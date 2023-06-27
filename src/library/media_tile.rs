use std::collections::VecDeque;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self, gdk, gdk_pixbuf},
    loading_widgets::LoadingWidgets,
    view, AsyncComponentSender,
};

use crate::{app::APP_BROKER, jellyfin_api::models::media::Media};

#[derive(Clone, Copy)]
pub enum MediaTileDisplay {
    Cover,
    Wide,
}

impl MediaTileDisplay {
    fn width(&self) -> i32 {
        match self {
            Self::Cover => 133,
            Self::Wide => 263,
        }
    }

    fn height(&self) -> i32 {
        match self {
            Self::Cover => 200,
            Self::Wide => 150,
        }
    }
}

pub struct MediaTile {
    media: Media,
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaTile {
    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = (Media, MediaTileDisplay);

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "media-tile",

            add_controller = gtk::GestureClick {
                connect_pressed[media] => move |_, _, _, _| {
                    APP_BROKER.send(crate::app::AppInput::PlayVideo(media.clone()));
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

            gtk::Overlay {
                set_halign: gtk::Align::Center,

                #[name = "image"]
                gtk::Picture {
                    set_content_fit: gtk::ContentFit::Cover,
                    set_can_shrink: true,

                    set_width_request: tile_display.width(),
                    set_height_request: tile_display.height(),
                },

                add_overlay = &gtk::ProgressBar {
                    set_valign: gtk::Align::End,
                    set_visible: model.media.user_data.played_percentage.is_some(),
                    set_fraction?: model.media.user_data.played_percentage.map(|p| p / 100.0),
                    set_overflow: gtk::Overflow::Hidden,
                },
            },

            gtk::Label {
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                #[watch]
                set_label: &model.media.name,
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
        let (media, tile_display) = init;

        let img_url = media.image_tags.primary.clone();

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
        let paintable = gdk::Texture::for_pixbuf(&pixbuf);

        let model = MediaTile {
            media: media.clone(),
        };

        let widgets = view_output!();
        let image = &widgets.image;

        image.set_paintable(Some(&paintable));

        AsyncComponentParts { model, widgets }
    }
}
