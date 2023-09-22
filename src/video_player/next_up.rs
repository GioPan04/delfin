use std::{collections::VecDeque, sync::Arc};

use gtk::prelude::*;
use relm4::{
    gtk::{
        gdk::{self, Texture},
        gdk_pixbuf,
    },
    prelude::*,
    SharedState,
};

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::models::media::Media,
};

use super::gst_play_widget::GstVideoPlayer;

// How many seconds should be remaining when the next up popup appears
const SHOW_NEXT_UP_AT: u64 = 30;

pub(crate) static NEXT_UP_VISIBILE: SharedState<bool> = SharedState::new();

#[derive(Debug)]
pub(crate) struct NextUp {
    state: NextUpState,
    next_up: Option<Media>,
    duration: Option<gst::ClockTime>,
    thumbnail: Option<Texture>,
}

#[derive(Debug)]
enum NextUpState {
    Ready,
    Shown(u64),
    Hidden,
}

#[derive(Debug)]
pub(crate) enum NextUpInput {
    Reset,
    SetNextUp(Box<Option<Media>>),
    SetDuration(gst::ClockTime),
    SetPosition(gst::ClockTime),
    PlayNext,
    Hide,
}

#[derive(Debug)]
pub(crate) enum NextUpCommandOutput {
    SetThumbnail(VecDeque<u8>),
}

#[relm4::component(pub(crate))]
impl Component for NextUp {
    type Init = Arc<GstVideoPlayer>;
    type Input = NextUpInput;
    type Output = ();
    type CommandOutput = NextUpCommandOutput;

    view! {
        gtk::Box {
            #[watch]
            set_visible: matches!(model.state, NextUpState::Shown(_)),

            add_css_class: "osd",
            add_css_class: "next-up",
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_halign: gtk::Align::End,
            set_valign: gtk::Align::End,
            set_margin_end: 24,
            set_margin_bottom: 24,

            gtk::Label {
                #[watch]
                set_label: &format!("Next episode starting in {} seconds...", if let NextUpState::Shown(remaining) = model.state {
                    remaining
                } else { 0 }),

                add_css_class: "body",
                set_halign: gtk::Align::Start,
            },

            gtk::Picture {
                #[watch]
                set_paintable: model.thumbnail.as_ref(),

                add_css_class: "next-up__thumbnail",
                set_width_request: 300,
            },

            gtk::Box {
                add_css_class: "linked",
                set_orientation: gtk::Orientation::Horizontal,
                set_homogeneous: true,

                gtk::Button {
                    add_css_class: "suggested-action",
                    adw::ButtonContent {
                        set_icon_name: "play-filled",
                        set_label: "Play now",
                    },

                    connect_clicked[sender] => move |_| {
                        sender.input(NextUpInput::PlayNext);
                    },
                },

                gtk::Button {
                    set_label: "Hide",

                    connect_clicked[sender] => move |_| {
                        sender.input(NextUpInput::Hide);
                    },
                },
            },
        }
    }

    fn init(
        video_player: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NextUp {
            state: NextUpState::Ready,
            next_up: None,
            duration: None,
            thumbnail: None,
        };

        video_player.connect_position_updated({
            let sender = sender.clone();
            move |position| {
                sender.input(NextUpInput::SetPosition(position));
            }
        });

        video_player.connect_duration_changed({
            let sender = sender.clone();
            move |duration| {
                sender.input(NextUpInput::SetDuration(duration));
            }
        });

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            NextUpInput::Reset => {
                self.state = NextUpState::Ready;
                self.next_up = None;
                self.duration = None;
                self.set_visible(false);
            }
            NextUpInput::SetNextUp(next) => {
                self.next_up = *next;
                self.fetch_next_up_thumbnail(&sender);
            }
            NextUpInput::SetDuration(duration) => {
                self.duration = Some(duration);
            }
            NextUpInput::SetPosition(position) => match self.state {
                NextUpState::Ready => {
                    if let (Some(_), Some(duration)) = (&self.next_up, &self.duration) {
                        if duration.saturating_sub(position).seconds() <= SHOW_NEXT_UP_AT {
                            self.set_visible(true);
                        }
                    }
                }
                NextUpState::Shown(_) => {
                    if let Some(duration) = &self.duration {
                        self.state =
                            NextUpState::Shown(duration.saturating_sub(position).seconds());
                    }
                }
                _ => {}
            },
            NextUpInput::PlayNext => {
                if let Some(next_up) = &self.next_up {
                    APP_BROKER.send(AppInput::PlayVideo(next_up.clone()));
                }
            }
            NextUpInput::Hide => {
                self.set_visible(false);
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let NextUpCommandOutput::SetThumbnail(img_bytes) = message;
        let pixbuf = gdk_pixbuf::Pixbuf::from_read(img_bytes)
            .expect("Error creating media tile pixbuf: {img_url}");
        self.thumbnail = Some(gdk::Texture::for_pixbuf(&pixbuf));
    }
}

impl NextUp {
    fn set_visible(&mut self, visible: bool) {
        if visible {
            self.state = NextUpState::Shown(SHOW_NEXT_UP_AT);
        } else if let NextUpState::Shown(_) = self.state {
            self.state = NextUpState::Hidden;
        }
        *NEXT_UP_VISIBILE.write() = visible;
    }

    fn fetch_next_up_thumbnail(&mut self, sender: &ComponentSender<Self>) {
        if let Some(next_up) = &self.next_up {
            let img_url = next_up.image_tags.primary.clone();
            sender.oneshot_command(async {
                let img_bytes: VecDeque<u8> = reqwest::get(img_url)
                    .await
                    .expect("Error getting media tile image: {img_url}")
                    .bytes()
                    .await
                    .expect("Error getting media tile image bytes: {img_url}")
                    .into_iter()
                    .collect();
                NextUpCommandOutput::SetThumbnail(img_bytes)
            });
        }
    }
}
