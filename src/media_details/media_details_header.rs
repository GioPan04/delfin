use std::{collections::VecDeque, matches, sync::Arc};

use adw::{prelude::*, SqueezerTransitionType};
use relm4::{
    gtk::{gdk::Texture, gdk_pixbuf::Pixbuf},
    prelude::*,
};

use crate::jellyfin_api::{
    api::{
        item::{GetItemRes, ItemType},
        latest::GetNextUpOptionsBuilder,
    },
    api_client::ApiClient,
    models::media::Media,
};

pub const MEDIA_DETAILS_BACKDROP_HEIGHT: i32 = 400;

#[derive(Debug)]
pub(crate) struct MediaDetailsHeader {
    media: Media,
    backdrop: Option<Texture>,
    play_next_label: Option<String>,
}

pub(crate) struct MediaDetailsHeaderInit {
    pub(crate) api_client: Arc<ApiClient>,
    pub(crate) media: Media,
    pub(crate) item: GetItemRes,
}

#[derive(Debug)]
pub enum MediaDetailsHeaderCommandOutput {
    PlayNextLoaded(String),
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
                            #[watch]
                            set_sensitive: model.play_next_label.is_some(),

                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 8,

                                if model.play_next_label.is_some() {
                                    gtk::Label {
                                        #[watch]
                                        set_label: model.play_next_label.as_ref().unwrap(),
                                    }
                                } else { gtk::Spinner { set_spinning: true } },

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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let MediaDetailsHeaderInit {
            api_client,
            media,
            item,
        } = init;

        let img_url = match &item.backdrop_image_urls {
            Some(backdrop_image_urls) => {
                if !backdrop_image_urls.is_empty() {
                    Some(backdrop_image_urls[0].clone())
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(img_url) = img_url {
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

        sender.oneshot_command({
            let media = media.clone();
            async move {
                let play_next_label = get_next_episode_btn_label(&api_client, &item, &media).await;
                MediaDetailsHeaderCommandOutput::PlayNextLoaded(play_next_label)
            }
        });

        let model = MediaDetailsHeader {
            media,
            backdrop: None,
            play_next_label: None,
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
            MediaDetailsHeaderCommandOutput::PlayNextLoaded(play_next) => {
                self.play_next_label = Some(play_next);
            }
            MediaDetailsHeaderCommandOutput::BackdropLoaded(img_bytes) => {
                let pixbuf = Pixbuf::from_read(img_bytes)
                    .expect("Error creating media tile pixbuf: {img_url}");
                self.backdrop = Some(Texture::for_pixbuf(&pixbuf));
            }
        }
    }
}

// Keep away from this accursed function
async fn get_next_episode_btn_label(
    api_client: &Arc<ApiClient>,
    item: &GetItemRes,
    media: &Media,
) -> String {
    let verb = if media.user_data.playback_position_ticks == 0 {
        "Play"
    } else {
        "Resume"
    }
    .to_string();

    if !(matches!(media.media_type, ItemType::Episode)
        || matches!(media.media_type, ItemType::Series))
    {
        return verb;
    }

    if let (Some(parent_index_number), Some(index_number)) =
        (&media.parent_index_number, &media.index_number)
    {
        return format!("{verb} S{parent_index_number}:E{index_number}");
    }

    let next_up = match api_client
        .get_next_up(
            GetNextUpOptionsBuilder::default()
                .series_id(&item.id)
                .build()
                .unwrap(),
        )
        .await
        .map(|next_up| {
            if !next_up.is_empty() {
                return Some(next_up[0].clone());
            }
            None
        }) {
        Ok(Some(next_up)) => next_up,
        _ => return verb,
    };

    if let (Some(parent_index_number), Some(index_number)) =
        (&next_up.parent_index_number, &next_up.index_number)
    {
        return format!("{verb} S{parent_index_number}:E{index_number}");
    }

    "Play next episode".to_string()
}
