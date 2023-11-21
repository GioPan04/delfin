use std::{collections::VecDeque, sync::Arc};

use gdk::Texture;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::{
    gtk::{self, gdk, gdk_pixbuf, prelude::*},
    prelude::{AsyncComponent, AsyncComponentParts},
    AsyncComponentSender,
};
use uuid::Uuid;

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::{
        api::{latest::GetNextUpOptionsBuilder, shows::GetEpisodesOptionsBuilder},
        api_client::ApiClient,
    },
    tr,
    utils::item_name::ItemName,
};

use super::LIBRARY_BROKER;

#[derive(Clone, Copy)]
pub enum MediaTileDisplay {
    Cover,
    Wide,
}

impl MediaTileDisplay {
    pub fn width(&self) -> i32 {
        match self {
            Self::Cover => 133,
            Self::Wide => 263,
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            Self::Cover => 200,
            Self::Wide => 150,
        }
    }
}

fn get_item_label(item: &BaseItemDto) -> String {
    match (&item.series_name, item.episode_name_with_number()) {
        (Some(series_name), Some(name)) => format!(
            r#"{series_name}
<span size="small">{name}</span>"#
        ),
        (_, Some(name)) => name,
        _ => tr!("library-media-tile-unnamed-item").to_string(),
    }
}

pub struct MediaTile {
    media: BaseItemDto,
    api_client: Arc<ApiClient>,
    thumbnail: Option<Texture>,
}

#[derive(Debug)]
pub enum MediaTileInput {
    Play,
}

#[derive(Debug)]
pub enum MediaTileCommandOutput {
    ThumbnailLoaded(Option<Texture>),
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaTile {
    type Init = (BaseItemDto, MediaTileDisplay, Arc<ApiClient>);
    type Input = MediaTileInput;
    type Output = ();
    type CommandOutput = MediaTileCommandOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Center,
            set_spacing: 8,
            add_css_class: "media-tile",


            #[name = "overlay"]
            gtk::Overlay {
                set_halign: gtk::Align::Start,
                set_cursor_from_name: Some("pointer"),

                // TODO: progress bar overflows media tile. Hiding the overflow is a workaround, but it makes the
                // progress bar percentage look wrong.
                set_overflow: gtk::Overflow::Hidden,


                add_controller = gtk::GestureClick {
                    connect_pressed[sender] => move |_, _, _, _| {
                        sender.input(MediaTileInput::Play);
                    },
                },

                add_controller = gtk::EventControllerMotion {
                    connect_enter[root] => move |_, _, _| {
                        root.add_css_class("hover");
                    },
                    connect_leave[root] => move |_| {
                        root.remove_css_class("hover");
                    },
                },

                #[name = "image"]
                gtk::Picture {
                    #[watch]
                    set_paintable: model.thumbnail.as_ref(),

                    set_content_fit: gtk::ContentFit::Cover,
                    set_width_request: tile_display.width(),
                    set_height_request: tile_display.height(),
                },

                add_overlay = &gtk::Spinner {
                    #[watch]
                    set_visible: model.thumbnail.is_none(),
                    start: (),
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_width_request: 32,
                    set_height_request: 32,
                },

                add_overlay = &gtk::CenterBox {
                    add_css_class: "hover-overlay",

                    set_halign: gtk::Align::Fill,
                    set_valign: gtk::Align::Fill,

                    #[wrap(Some)]
                    set_center_widget = &gtk::Image {
                        set_from_icon_name: Some("play-filled"),

                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                    }
                },

                add_overlay = &gtk::ProgressBar {
                    set_valign: gtk::Align::End,
                    set_visible: model.media.user_data.as_ref().map(|user_data| user_data.played_percentage).is_some(),
                    set_fraction?: model.media.user_data.as_ref()
                        .and_then(|user_data| user_data.played_percentage)
                        .map(|played_percentage| played_percentage / 100.0),
                    set_overflow: gtk::Overflow::Hidden,
                    set_width_request: tile_display.width(),
                },
            },

            gtk::Label {
                set_halign: gtk::Align::Fill,
                set_justify: gtk::Justification::Center,
                set_cursor_from_name: Some("pointer"),
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                set_max_width_chars: 1,
                #[watch]
                set_markup: &get_item_label(&model.media),

                add_controller = gtk::GestureClick {
                    connect_pressed => move |_, _, _, _| {
                        APP_BROKER.send(AppInput::ShowDetails(media.clone()));
                    },
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let (media, tile_display, api_client) = init;

        let model = MediaTile {
            media: media.clone(),
            api_client: api_client.clone(),
            thumbnail: None,
        };

        let widgets = view_output!();

        sender.oneshot_command({
            let media = model.media.clone();
            async move {
                let thumbnail = get_thumbnail(api_client, &media, &tile_display).await;
                MediaTileCommandOutput::ThumbnailLoaded(thumbnail)
            }
        });

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MediaTileInput::Play => {
                match get_next_playable_media(self.api_client.clone(), self.media.clone()).await {
                    Some(media) => APP_BROKER.send(AppInput::PlayVideo(media)),
                    _ => {
                        let mut message = "No playable media found".to_string();
                        if let Some(name) = self.media.name.as_ref() {
                            message += &format!(" for {name}");
                        }
                        LIBRARY_BROKER.send(super::LibraryInput::Toast(message))
                    }
                };
            }
        }
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MediaTileCommandOutput::ThumbnailLoaded(thumbnail) => {
                self.thumbnail = thumbnail;
            }
        }
    }
}

async fn get_thumbnail(
    api_client: Arc<ApiClient>,
    media: &BaseItemDto,
    tile_display: &MediaTileDisplay,
) -> Option<gdk::Texture> {
    let img_url = match tile_display {
        MediaTileDisplay::Wide => api_client.get_parent_or_item_backdrop_url(media),
        MediaTileDisplay::Cover => api_client.get_parent_or_item_thumbnail_url(media),
    };
    let img_url = match img_url {
        Ok(img_url) => img_url,
        _ => return None,
    };

    let img_bytes: VecDeque<u8> = reqwest::get(img_url)
        .await
        .expect("Error getting media tile image: {img_url}")
        .bytes()
        .await
        .expect("Error getting media tile image bytes: {img_url}")
        .into_iter()
        .collect();

    let pixbuf = gdk_pixbuf::Pixbuf::from_read(img_bytes)
        .unwrap_or_else(|_| panic!("Error creating media tile pixbuf: {:#?}", media.id));
    Some(gdk::Texture::for_pixbuf(&pixbuf))
}

// Gets the next playable media for the given media item.
// For episodes and movies, this just returns the passed in media.
// For TV shows, this looks for the next episode for the user to start/continue watching the
// series.
async fn get_next_playable_media(
    api_client: Arc<ApiClient>,
    media: BaseItemDto,
) -> Option<BaseItemDto> {
    let media_id = media.id.expect("Media missing id: {media:#?}");
    let media_type = media.type_.expect("Media missing type: {media:#?}");

    match media_type {
        BaseItemKind::Series => get_next_episode(api_client, media_id).await,
        _ => Some(media),
    }
}

async fn get_next_episode(api_client: Arc<ApiClient>, media_id: Uuid) -> Option<BaseItemDto> {
    if let Some(resume) = api_client
        .get_continue_watching(
            GetNextUpOptionsBuilder::default()
                .series_id(media_id)
                .limit(1)
                .build()
                .unwrap(),
        )
        .await
        .as_ref()
        .ok()
        .and_then(|resume| resume.first())
    {
        return Some(resume.to_owned());
    };

    if let Some(next_up) = api_client
        .get_next_up(
            GetNextUpOptionsBuilder::default()
                .series_id(media_id)
                .limit(1)
                .build()
                .unwrap(),
        )
        .await
        .ok()
        .as_ref()
        .and_then(|next_up| next_up.first())
    {
        return Some(next_up.to_owned());
    }

    if let Some(items) = api_client
        .get_episodes(
            &GetEpisodesOptionsBuilder::default()
                .series_id(media_id)
                .build()
                .unwrap(),
        )
        .await
        .ok()
        .as_ref()
        .and_then(|episodes| episodes.first())
    {
        return Some(items.to_owned());
    }

    None
}
