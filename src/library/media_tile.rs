use std::{collections::VecDeque, sync::Arc};

use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self, gdk, gdk_pixbuf},
    loading_widgets::LoadingWidgets,
    view, AsyncComponentSender,
};

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::api_client::ApiClient,
};

pub const MEDIA_TILE_COVER_WIDTH: i32 = 133;

#[derive(Clone, Copy)]
pub enum MediaTileDisplay {
    Cover,
    Wide,
}

impl MediaTileDisplay {
    fn width(&self) -> i32 {
        match self {
            Self::Cover => MEDIA_TILE_COVER_WIDTH,
            Self::Wide => 263,
        }
    }

    fn height(&self) -> i32 {
        match self {
            Self::Cover => 200,
            Self::Wide => 150,
        }
    }

    fn label_halign(&self) -> gtk::Align {
        match self {
            Self::Cover => gtk::Align::Fill,
            Self::Wide => gtk::Align::Start,
        }
    }

    fn label_max_width_characters(&self) -> i32 {
        match self {
            Self::Cover => 1,
            Self::Wide => -1,
        }
    }
}

fn get_item_label(item: &BaseItemDto) -> String {
    if let Some(name) = &item.name {
        if let (Some(series_name), Some(index_number), Some(parent_index_number)) = (
            &item.series_name,
            &item.index_number,
            &item.parent_index_number,
        ) {
            return format!(
                r#"{series_name}
<span size="small">S{parent_index_number}:E{index_number} - {}</span>"#,
                name
            );
        }

        return format!("{}\n", name);
    }

    "Unnamed Item".to_string()
}

pub struct MediaTile {
    media: BaseItemDto,
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaTile {
    type Init = (BaseItemDto, MediaTileDisplay, Arc<ApiClient>);
    type CommandOutput = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Start,
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
                    connect_pressed[media] => move |_, _, _, _| {
                        APP_BROKER.send(AppInput::PlayVideo(media.clone()));
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
                    set_content_fit: gtk::ContentFit::Cover,
                    // set_can_shrink: true,

                    set_width_request: tile_display.width(),
                    set_height_request: tile_display.height(),
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
                set_halign: tile_display.label_halign(),
                set_cursor_from_name: Some("pointer"),
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                set_max_width_chars: tile_display.label_max_width_characters(),
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

    fn init_loading_widgets(
        root: &mut Self::Root,
    ) -> Option<relm4::loading_widgets::LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_hexpand: true,
                    set_vexpand: true,
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let (media, tile_display, api_client) = init;

        let model = MediaTile {
            media: media.clone(),
        };

        let widgets = view_output!();
        let image = &widgets.image;

        let paintable = get_thumbnail(api_client, &model.media, &tile_display).await;
        image.set_paintable(paintable.as_ref());

        AsyncComponentParts { model, widgets }
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
        .expect("Error creating media tile pixbuf: {img_url}");
    Some(gdk::Texture::for_pixbuf(&pixbuf))
}
