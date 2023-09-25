use std::{collections::VecDeque, matches, sync::Arc};

use adw::{prelude::*, BreakpointCondition};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::{
    gtk::{gdk::Texture, gdk_pixbuf::Pixbuf},
    prelude::*,
};

use crate::{
    app::APP_BROKER,
    jellyfin_api::{api::latest::GetNextUpOptionsBuilder, api_client::ApiClient},
};

pub const MEDIA_DETAILS_BACKDROP_HEIGHT: i32 = 400;

#[derive(Debug)]
pub(crate) struct MediaDetailsHeader {
    media: BaseItemDto,
    backdrop: Option<Texture>,
    play_next_label: Option<String>,
    play_next_media: Option<BaseItemDto>,
}

pub(crate) struct MediaDetailsHeaderInit {
    pub(crate) api_client: Arc<ApiClient>,
    pub(crate) media: BaseItemDto,
    pub(crate) item: BaseItemDto,
}

#[derive(Debug)]
pub enum MediaDetailsHeaderInput {
    PlayNext,
}

#[derive(Debug)]
pub enum MediaDetailsHeaderCommandOutput {
    PlayNextLoaded(Box<(String, Option<BaseItemDto>)>),
    BackdropLoaded(VecDeque<u8>),
}

#[relm4::component(pub(crate))]
impl Component for MediaDetailsHeader {
    type Init = MediaDetailsHeaderInit;
    type Input = MediaDetailsHeaderInput;
    type Output = ();
    type CommandOutput = MediaDetailsHeaderCommandOutput;

    view! {
        adw::BreakpointBin {
            set_size_request: (150, MEDIA_DETAILS_BACKDROP_HEIGHT),

            #[wrap(Some)]
            set_child = &gtk::Overlay {
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

                        #[name = "fade_overlay"]
                        add_overlay = &gtk::Box {
                            set_width_request: 1280,
                            // This needs to be hidden by default for the breakpoint to show it
                            // properly, so we dynamically chang the visibility of the children
                            set_visible: false,

                            gtk::Box {
                                add_css_class: "media-details-backdrop-overlay-left",
                                set_width_request: 32,
                                set_halign: gtk::Align::Start,
                                set_valign: gtk::Align::Fill,
                                #[watch]
                                set_visible: model.backdrop.is_some(),
                            },

                            gtk::Box {
                                add_css_class: "media-details-backdrop-overlay-right",
                                set_width_request: 32,
                                set_halign: gtk::Align::End,
                                set_valign: gtk::Align::Fill,
                                set_hexpand: true,
                                #[watch]
                                set_visible: model.backdrop.is_some(),
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
                                set_label: &title,
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
                                set_sensitive: model.play_next_label.is_some() && model.play_next_media.is_some(),

                                connect_clicked[sender] => move |_| {
                                    sender.input(MediaDetailsHeaderInput::PlayNext);
                                },

                                #[wrap(Some)]
                                set_child = &gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 8,

                                    gtk::Image::from_icon_name("play-filled"),

                                    if model.play_next_label.is_some() {
                                        gtk::Label {
                                            #[watch]
                                            set_label: model.play_next_label.as_ref().unwrap(),
                                        }
                                    } else { gtk::Spinner { set_spinning: true } },
                                },
                            },
                        },

                    },
                },
            },

            add_breakpoint = adw::Breakpoint::new(BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MinWidth,
                1280.0,
                adw::LengthUnit::Sp
            )) {
                add_setter: (&fade_overlay, "visible", &true.into()),
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

        if let Ok(img_url) = api_client.get_backdrop_url(&item) {
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
                let play_next = get_play_next(&api_client, &item, &media).await;
                MediaDetailsHeaderCommandOutput::PlayNextLoaded(Box::new(play_next))
            }
        });

        let model = MediaDetailsHeader {
            media,
            backdrop: None,
            play_next_label: None,
            play_next_media: None,
        };

        let title = model
            .media
            .series_name
            .clone()
            .or(model.media.name.clone())
            .unwrap_or("Unnamed Item".to_string());

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            MediaDetailsHeaderInput::PlayNext => {
                if let Some(play_next_media) = &self.play_next_media {
                    APP_BROKER.send(crate::app::AppInput::PlayVideo(play_next_media.clone()));
                }
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MediaDetailsHeaderCommandOutput::PlayNextLoaded(play_next) => {
                let (play_next_label, play_next_media) = *play_next;
                self.play_next_label = Some(play_next_label);
                self.play_next_media = play_next_media;
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
async fn get_play_next(
    api_client: &Arc<ApiClient>,
    item: &BaseItemDto,
    media: &BaseItemDto,
) -> (String, Option<BaseItemDto>) {
    let verb = if media
        .user_data
        .clone()
        .and_then(|user_data| user_data.playback_position_ticks)
        .unwrap_or(0)
        == 0
    {
        "Play"
    } else {
        "Resume"
    }
    .to_string();

    if !(matches!(
        media.type_,
        Some(BaseItemKind::Episode) | Some(BaseItemKind::Series)
    )) {
        return (verb, Some(media.clone()));
    }

    if let (Some(parent_index_number), Some(index_number)) =
        (&media.parent_index_number, &media.index_number)
    {
        return (
            format!("{verb} S{parent_index_number}:E{index_number}"),
            Some(media.clone()),
        );
    }

    if let Some(series_id) = item.id {
        let next_up = match api_client
            .get_next_up(
                GetNextUpOptionsBuilder::default()
                    .series_id(series_id)
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
            Ok(Some(next_up)) => Some(next_up),
            _ => None,
        };

        if let Some(next_up) = next_up {
            if let (Some(parent_index_number), Some(index_number)) =
                (&next_up.parent_index_number, &next_up.index_number)
            {
                return (
                    format!("{verb} S{parent_index_number}:E{index_number}"),
                    Some(next_up),
                );
            }
        }
    }

    ("Play next episode".to_string(), None)
}
