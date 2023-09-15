use std::collections::VecDeque;

use adw::{prelude::*, SqueezerTransitionType};
use relm4::{
    gtk::{gdk::Texture, gdk_pixbuf::Pixbuf},
    prelude::*,
    Component, ComponentParts,
};

use crate::jellyfin_api::{api::item::GetItemRes, models::media::Media};

pub const MEDIA_DETAILS_BACKDROP_HEIGHT: i32 = 400;

#[derive(Debug)]
pub(crate) struct MediaDetailsHeader {
    media: Media,
    backdrop: Option<Texture>,
}

pub(crate) struct MediaDetailsHeaderInit {
    pub(crate) media: Media,
    pub(crate) item: GetItemRes,
}

#[derive(Debug)]
pub enum MediaDetailsHeaderCommandOutput {
    BackdropLoaded(VecDeque<u8>),
}

#[relm4::component(pub(crate))]
impl Component for MediaDetailsHeader {
    type Init = MediaDetailsHeaderInit;
    type Input = ();
    type Output = ();
    type CommandOutput = MediaDetailsHeaderCommandOutput;

    view! {
        gtk::Overlay {
            set_height_request: MEDIA_DETAILS_BACKDROP_HEIGHT,
            add_css_class: "media-details-header",
            set_overflow: gtk::Overflow::Hidden,

            // Leaving this here for now, might come back to this later
            // gtk::Picture {
            //     #[watch]
            //     set_paintable: model.backdrop.as_ref(),
            //
            //     add_css_class: "media-details-backdrop-blur",
            //     set_halign: gtk::Align::Center,
            //     set_valign: gtk::Align::Center,
            //     set_content_fit: gtk::ContentFit::Fill,
            // },

            add_overlay = &adw::Clamp {
                set_maximum_size: 1280,
                set_tightening_threshold: 1280,
                connect_maximum_size_notify => |_| {},

                gtk::Overlay {
                    gtk::Picture {
                        #[watch]
                        set_paintable: model.backdrop.as_ref(),

                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        set_height_request: MEDIA_DETAILS_BACKDROP_HEIGHT,
                        set_content_fit: gtk::ContentFit::Cover,
                    },

                    add_overlay = &gtk::Spinner {
                        #[watch]
                        set_visible: model.backdrop.is_none(),
                        set_spinning: true,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        set_width_request: 24,
                        set_height_request: 24,
                    },

                    add_overlay = &adw::Squeezer {
                        set_allow_none: true,
                        set_transition_type: SqueezerTransitionType::Crossfade,

                        gtk::Box {
                            set_width_request: 1280,

                            gtk::Box {
                                add_css_class: "media-details-backdrop-overlay-left",
                                set_width_request: 32,
                                set_halign: gtk::Align::Start,
                                set_valign: gtk::Align::Fill,
                            },

                            gtk::Box {
                                add_css_class: "media-details-backdrop-overlay-right",
                                set_width_request: 32,
                                set_halign: gtk::Align::End,
                                set_valign: gtk::Align::Fill,
                                set_hexpand: true,
                            },
                        },
                    },
                },
            },

            add_overlay = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                add_css_class: "media-details-header-overlay",

                adw::Clamp {
                    set_maximum_size: 1280,
                    set_tightening_threshold: 1280,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_valign: gtk::Align::End,
                        set_margin_start:  32,
                        set_margin_end: 32,
                        set_spacing: 32,

                        gtk::Label {
                            set_label: title,
                            set_valign: gtk::Align::Center,
                            set_ellipsize: gtk::pango::EllipsizeMode::End,
                            add_css_class: "media-details-header-title",
                        },

                        gtk::Button {
                            add_css_class: "pill",
                            add_css_class: "suggested-action",
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::Center,
                            set_hexpand: true,
                            set_vexpand: false,

                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 8,

                                gtk::Label::new(Some("Play next episode")),
                                gtk::Image::from_icon_name("play-filled"),
                            },
                        },
                    },

                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let MediaDetailsHeaderInit { media, item } = init;

        let backdrop_image_urls = item.backdrop_image_urls.unwrap_or(vec![]);
        if !backdrop_image_urls.is_empty() {
            let img_url = backdrop_image_urls[0].clone();
            sender.oneshot_command(async {
                let img_bytes: VecDeque<u8> = reqwest::get(img_url)
                    .await
                    .expect("Error getting media tile image: {img_url}")
                    .bytes()
                    .await
                    .expect("Error getting media tile image bytes: {img_url}")
                    .into_iter()
                    .collect();
                MediaDetailsHeaderCommandOutput::BackdropLoaded(img_bytes)
            });
        }

        let model = MediaDetailsHeader {
            media,
            backdrop: None,
        };

        let title = &model
            .media
            .series_name
            .as_ref()
            .unwrap_or(&model.media.name);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MediaDetailsHeaderCommandOutput::BackdropLoaded(img_bytes) => {
                let pixbuf = Pixbuf::from_read(img_bytes)
                    .expect("Error creating media tile pixbuf: {img_url}");
                self.backdrop = Some(Texture::for_pixbuf(&pixbuf));
            }
        }
    }
}
