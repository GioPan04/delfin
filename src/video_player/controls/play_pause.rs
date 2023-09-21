use std::cell::OnceCell;

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, MessageBroker, SimpleComponent};

use crate::video_player::gst_play_widget::GstVideoPlayer;

pub(crate) struct PlayPause {
    video_player: OnceCell<GstVideoPlayer>,
    loading: bool,
    playing: bool,
}

pub static PLAY_PAUSE_BROKER: MessageBroker<PlayPauseInput> = MessageBroker::new();

#[derive(Debug)]
pub enum PlayPauseInput {
    TogglePlaying,
    SetLoading,
    SetPlaying(bool),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for PlayPause {
    type Init = OnceCell<GstVideoPlayer>;
    type Input = PlayPauseInput;
    type Output = ();

    view! {
        gtk::Button {
            #[watch]
            set_icon_name: if model.playing {
                "pause-filled"
            } else {
                "play-filled"
            },
            #[watch]
            set_tooltip_text: Some(if model.playing {
                "Pause"
            } else {
                "Play"
            }),
            #[watch]
            set_sensitive: !model.loading,
            connect_clicked[sender] => move |_| {
                sender.input(PlayPauseInput::TogglePlaying);
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PlayPause {
            video_player: init,
            loading: true,
            playing: true,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PlayPauseInput::TogglePlaying => {
                let video_player = self.video_player.get().unwrap();
                match self.playing {
                    true => {
                        video_player.pause();
                        self.playing = false;
                    }
                    false => {
                        video_player.play();
                        self.playing = true;
                    }
                }
            }
            PlayPauseInput::SetLoading => {
                self.loading = true;
            }
            PlayPauseInput::SetPlaying(playing) => {
                self.playing = playing;
                self.loading = false;
            }
        }
    }
}
