use std::collections::VecDeque;
use std::sync::Arc;

use gdk::Texture;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::{
    gtk::{self, gdk, gdk_pixbuf, glib::markup_escape_text, prelude::*},
    prelude::{AsyncComponent, AsyncComponentParts},
    AsyncComponentSender,
};
use tracing::{debug, error};

use crate::{
    app::{AppInput, APP_BROKER},
    globals::CONFIG,
    jellyfin_api::api_client::ApiClient,
    tr,
    utils::{display_years::DisplayYears, item_name::ItemName, playable::get_next_playable_media},
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
    pub fn width(&self) -> u16 {
        match self {
            Self::Cover => 133,
            Self::CoverLarge => 175,
            Self::Wide => 300,
            Self::CollectionWide => 300,
            Self::Buttons => 300,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            Self::Cover => 200,
            Self::CoverLarge => 262,
            Self::Wide => 175,
            Self::CollectionWide => 175,
            Self::Buttons => 0,
        }
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

                set_accessible_role: gtk::AccessibleRole::Button,
                set_focusable: true,
                connect_has_focus_notify[root, media] => move |overlay| {
                    if overlay.has_focus() && !matches!(media.type_, Some(BaseItemKind::CollectionFolder)) {
                        root.add_css_class("hover");
                    } else {
                        root.remove_css_class("hover");
                    }
                },

                // TODO: progress bar overflows media tile. Hiding the overflow is a workaround, but it makes the
                // progress bar percentage look wrong.
                set_overflow: gtk::Overflow::Hidden,


                add_controller = gtk::GestureClick {
                    connect_released[sender] => move |_, _, _, _| {
                        sender.input(MediaTileInput::Play);
                    },
                },

                add_controller = gtk::EventControllerKey {
                    connect_key_released[sender] => move |_, key, _, _| {
                        if matches!(key, gdk::Key::Return | gdk::Key::space) {
                            sender.input(MediaTileInput::Play);
                        }
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
                    set_width_request: tile_display.width().into(),
                    set_height_request: tile_display.height().into(),
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
                    set_width_request: tile_display.width().into(),
                },
            },

            gtk::Label {
                set_halign: gtk::Align::Center,
                set_justify: gtk::Justification::Center,
                set_cursor_from_name: Some("pointer"),
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                set_max_width_chars: 1,
                set_width_request: tile_display.width().into(),
                #[watch]
                set_markup: &model.get_item_label(),

                set_has_tooltip: true,
                connect_query_tooltip[item_label] => move |_, _, _, _, tooltip| {
                    let label = gtk::Label::builder()
                        .use_markup(true)
                        .label(&item_label)
                        .justify(gtk::Justification::Center)
                        .build();
                    tooltip.set_custom(Some(&label));
                    true
                },

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

        let model = Self::new(media.clone(), api_client.clone());
        let item_label = model.get_item_label();

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
                                APP_BROKER.send(AppInput::Toast(message, None))
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

impl MediaTile {
    pub fn new(media: BaseItemDto, api_client: Arc<ApiClient>) -> Self {
        Self {
            media,
            api_client,
            thumbnail: None,
        }
    }

    fn get_item_label(&self) -> String {
        match (
            self.media
                .series_name
                .as_ref()
                .map(|s| markup_escape_text(s)),
            self.media
                .episode_name_with_number()
                .as_ref()
                .map(|s| markup_escape_text(s)),
            self.media.display_years(),
        ) {
            (Some(series_name), Some(name), _) => format!(
                r#"{series_name}
<span size="small">{name}</span>"#
            ),
            (_, Some(name), Some(display_years)) => format!(
                r#"{name}
<span size="small">{display_years}</span>"#
            ),
            (_, Some(name), _) => name.to_string(),
            _ => tr!("library-media-tile-unnamed-item").to_string(),
        }
    }
}

/// Query the API for the best thumbnail image for a media item
///
/// This request can fail for a number of IO reasons, but we don't care why it fails. It will be logged properly,
/// however we are only interested in whether we got an image at all. If not, whether it's because of an error or because
/// the requested item has no image associated, we return None so [`get_thumbnail`] can use a fallback icon.
///
/// TODO: This function should in the future use a typed media item, so that it is guaranteed URL generation cannot fail
/// due to misformed BaseItemDto.
async fn get_thumbnail_image(
    api_client: Arc<ApiClient>,
    media: &BaseItemDto,
    tile_display: &MediaTileDisplay,
) -> Option<VecDeque<u8>> {
    let img_url = match tile_display {
        MediaTileDisplay::Wide => {
            let aspect_ratio_good = media.primary_image_aspect_ratio.unwrap_or_default() > 1.5;

            if aspect_ratio_good && CONFIG.read().general.use_episode_image {
                api_client.get_episode_primary_image_url(media, tile_display.height())
            } else if aspect_ratio_good {
                api_client.get_parent_or_item_thumbnail_url(media, tile_display.height())
            } else {
                api_client.get_episode_thumbnail_or_backdrop_url(media, tile_display.height())
            }
        }
        MediaTileDisplay::Cover | MediaTileDisplay::CoverLarge => {
            api_client.get_parent_or_item_primary_image_url(media)
        }
        MediaTileDisplay::CollectionWide | MediaTileDisplay::Buttons => {
            api_client.get_collection_thumbnail_url(media)
        }
    };

    if let Some(img_url) = img_url {
        match api_client.get_image(&img_url).await {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                error!("Querying API for image failed due to error:\n{e}");
                None
            }
        }
    } else {
        debug!("No image for ID {}", media.id.unwrap());
        None
    }
}

// Provide a fallback icon for video/music/book depending on media.collection_type()
// async fn get_thumbnail_fallback(media: &BaseItemDto, tile_display: &MediaTileDisplay) -> Result<VecDeque<u8>>

/// Query the API for the best thumbnail image, or use a fallback icon.
///
/// See [`get_thumbnail_image`] and [`get_thumbnail_fallback`] for more details on those two cases.
/// TODO: This function should always return a gdk::Texture. For now it can return None when something fails
async fn get_thumbnail(
    api_client: Arc<ApiClient>,
    media: &BaseItemDto,
    tile_display: &MediaTileDisplay,
) -> Option<gdk::Texture> {
    let img_bytes = get_thumbnail_image(api_client, media, tile_display).await?;

    let pixbuf = match gdk_pixbuf::Pixbuf::from_read(img_bytes) {
        Ok(pixbuf) => pixbuf,
        _ => {
            error!(
                "Error creating media tile pixbuf {} ({})",
                media.id.unwrap(),
                media.name.as_deref().unwrap_or("UNKNOWN")
            );
            return None;
        }
    };

    // TODO: merge resizing with how it's done for episode list

    let resized = gdk_pixbuf::Pixbuf::new(
        gdk_pixbuf::Colorspace::Rgb,
        true,
        8,
        tile_display.width().into(),
        tile_display.height().into(),
    )?;

    let scale = tile_display.height() as f64 / pixbuf.height() as f64;

    pixbuf.scale(
        &resized,
        0,
        0,
        tile_display.width().into(),
        tile_display.height().into(),
        0.0,
        0.0,
        scale,
        scale,
        gdk_pixbuf::InterpType::Bilinear,
    );

    Some(gdk::Texture::for_pixbuf(&resized))
}
