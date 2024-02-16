use std::{collections::VecDeque, sync::Arc};

use gdk::Texture;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::{
    gtk::{self, gdk, gdk_pixbuf, glib::markup_escape_text, prelude::*},
    prelude::{AsyncComponent, AsyncComponentParts},
    AsyncComponentSender,
};
use tracing::error;

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::api_client::ApiClient,
    tr,
    utils::{item_name::ItemName, playable::get_next_playable_media},
};

#[derive(Clone, Copy)]
pub enum MediaTileDisplay {
    Cover,
    CoverLarge,
    Wide,
    CollectionWide,
    Buttons,
}

impl MediaTileDisplay {
    pub fn width(&self) -> i32 {
        match self {
            Self::Cover => 133,
            Self::CoverLarge => 175,
            Self::Wide => 300,
            Self::CollectionWide => 300,
            Self::Buttons => 300,
        }
    }

    pub fn height(&self) -> i32 {
        match self {
            Self::Cover => 200,
            Self::CoverLarge => 262,
            Self::Wide => 175,
            Self::CollectionWide => 175,
            Self::Buttons => 0,
        }
    }
}

fn get_item_label(item: &BaseItemDto) -> String {
    match (
        &item.series_name.as_ref().map(|s| markup_escape_text(s)),
        item.episode_name_with_number()
            .as_ref()
            .map(|s| markup_escape_text(s)),
    ) {
        (Some(series_name), Some(name)) => format!(
            r#"{series_name}
<span size="small">{name}</span>"#
        ),
        (_, Some(name)) => name.to_string(),
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
    ShowDetails,
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
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::Start,
            set_spacing: 8,
            add_css_class: "media-tile",


            #[name = "overlay"]
            gtk::Overlay {
                set_halign: gtk::Align::Center,
                set_cursor_from_name: Some("pointer"),

                // TODO: progress bar overflows media tile. Hiding the overflow is a workaround, but it makes the
                // progress bar percentage look wrong.
                set_overflow: gtk::Overflow::Hidden,


                add_controller = gtk::GestureClick {
                    connect_released[sender] => move |_, _, _, _| {
                        sender.input(MediaTileInput::Play);
                    },
                },

                add_controller = gtk::EventControllerMotion {
                    connect_enter[root, media] => move |_, _, _| {
                        if !matches!(media.type_, Some(BaseItemKind::CollectionFolder)) {
                            root.add_css_class("hover");
                        }
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
                set_halign: gtk::Align::Center,
                set_justify: gtk::Justification::Center,
                set_cursor_from_name: Some("pointer"),
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                set_max_width_chars: 1,
                set_width_request: tile_display.width(),
                #[watch]
                set_markup: &get_item_label(&model.media),

                add_controller = gtk::GestureClick {
                    connect_released[sender] => move |_, _, _, _| {
                        sender.input(MediaTileInput::ShowDetails);
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
                match self.media.type_ {
                    Some(BaseItemKind::CollectionFolder) => {
                        APP_BROKER.send(AppInput::ShowCollection(self.media.clone()));
                    }
                    _ => {
                        match get_next_playable_media(self.api_client.clone(), self.media.clone())
                            .await
                        {
                            Some(media) => APP_BROKER.send(AppInput::PlayVideo(media)),
                            _ => {
                                let mut message = "No playable media found".to_string();
                                if let Some(name) = self.media.name.as_ref() {
                                    message += &format!(" for {name}");
                                }
                                APP_BROKER.send(AppInput::Toast(message))
                            }
                        };
                    }
                };
            }
            MediaTileInput::ShowDetails => {
                match self.media.type_ {
                    Some(BaseItemKind::CollectionFolder) => {
                        APP_BROKER.send(AppInput::ShowCollection(self.media.clone()));
                    }
                    _ => {
                        APP_BROKER.send(AppInput::ShowDetails(self.media.clone()));
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
        MediaTileDisplay::Cover | MediaTileDisplay::CoverLarge => {
            api_client.get_parent_or_item_thumbnail_url(media)
        }
        MediaTileDisplay::CollectionWide | MediaTileDisplay::Buttons => {
            api_client.get_collection_thumbnail_url(media)
        }
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

    let pixbuf = match gdk_pixbuf::Pixbuf::from_read(img_bytes) {
        Ok(pixbuf) => pixbuf,
        _ => {
            error!("Error creating media tile pixbuf: {:#?}", media.id);
            return None;
        }
    };

    // TODO: merge resizing with how it's done for episode list

    let resized = gdk_pixbuf::Pixbuf::new(
        gdk_pixbuf::Colorspace::Rgb,
        true,
        8,
        tile_display.width(),
        tile_display.height(),
    )?;

    let scale = tile_display.height() as f64 / pixbuf.height() as f64;

    pixbuf.scale(
        &resized,
        0,
        0,
        tile_display.width(),
        tile_display.height(),
        0.0,
        0.0,
        scale,
        scale,
        gdk_pixbuf::InterpType::Bilinear,
    );

    Some(gdk::Texture::for_pixbuf(&resized))
}
