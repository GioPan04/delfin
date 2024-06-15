use std::{cell::RefCell, sync::Arc, time::Duration};

use gtk::prelude::*;
use relm4::{
    gtk,
    gtk::glib::{timeout_add, ControlFlow},
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::{
    tr, utils::message_broker::ResettableMessageBroker, video_player::backends::VideoPlayerBackend,
};

pub static PLAY_PAUSE_BROKER: ResettableMessageBroker<PlayPauseInput> =
    ResettableMessageBroker::new();

pub(crate) struct PlayPause {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    loading: bool,
    playing: bool,
    first_click: bool,
    double_click_time: Duration,
}

#[derive(Debug)]
pub enum PlayPauseInput {
    TogglePlaying,
    SetLoading,
    SetPlaying(bool),
    LeftClick,
    LeftClickTimeout,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for PlayPause {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = PlayPauseInput;
    type Output = ();

    view! {
        gtk::Button {
            set_focus_on_click: false,

            #[watch]
            set_icon_name: if model.playing {
                "pause-filled"
            } else {
                "play-filled"
            },
            #[watch]
            set_tooltip_text: Some(tr!(
                "vp-play-pause-tooltip",
                {"playing" => model.playing.to_string()},
            )),
            #[watch]
            set_sensitive: !model.loading,
            connect_clicked[sender] => move |_| {
                sender.input(PlayPauseInput::TogglePlaying);
            },
        }
    }

    fn init(
        video_player: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let double_click_time = gtk::Settings::default()
            .map(|s| s.gtk_double_click_time())
            .unwrap_or(400);
        let model = PlayPause {
            video_player,
            loading: true,
            playing: true,
            first_click: false,
            double_click_time: Duration::from_millis(double_click_time as u64),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PlayPauseInput::TogglePlaying => {
                if self.playing {
                    self.video_player.borrow().pause();
                    self.playing = false;
                } else {
                    self.video_player.borrow().play();
                    self.playing = true;
                }
            }
            PlayPauseInput::SetLoading => {
                self.loading = true;
            }
            PlayPauseInput::SetPlaying(playing) => {
                self.playing = playing;
                self.loading = false;
                if playing {
                    self.video_player.borrow().play();
                } else {
                    self.video_player.borrow().pause();
                }
            }
            PlayPauseInput::LeftClick => {
                if !self.first_click {
                    self.first_click = true;
                    _ = timeout_add(self.double_click_time, || {
                        PLAY_PAUSE_BROKER.send(PlayPauseInput::LeftClickTimeout);
                        ControlFlow::Break
                    });
                } else {
                    self.first_click = false;
                }
            }
            PlayPauseInput::LeftClickTimeout => {
                if self.first_click {
                    self.first_click = false;
                    sender.input(PlayPauseInput::TogglePlaying);
                }
            }
        }
    }
}
