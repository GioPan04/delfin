use std::collections::VecDeque;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self, gdk_pixbuf},
    loading_widgets::LoadingWidgets,
    view, AsyncComponentSender,
};

use crate::{app::APP_BROKER, jellyfin_api::models::media::Media};

pub struct MediaTile {
    media: Media,
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaTile {
    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = Media;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_width_request: 200,
            set_height_request: 256,
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
                #[name = "image"]
                gtk::Image {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_halign: gtk::Align::Fill,
                    set_valign: gtk::Align::Fill,
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
                set_width_request: 200,

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
        media: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
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

        let model = MediaTile {
            media: media.clone(),
        };

        let widgets = view_output!();
        let image = &widgets.image;

        image.set_from_pixbuf(Some(&pixbuf));

        AsyncComponentParts { model, widgets }
    }
}
