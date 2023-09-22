use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, MessageBroker, SimpleComponent};

use crate::video_player::gst_play_widget::GstVideoPlayer;

pub(crate) struct PlayPause {
    video_player: Arc<GstVideoPlayer>,
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
    type Init = Arc<GstVideoPlayer>;
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
        video_player: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PlayPause {
            video_player,
            loading: true,
            playing: true,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PlayPauseInput::TogglePlaying => match self.playing {
                true => {
                    self.video_player.pause();
                    self.playing = false;
                }
                false => {
                    self.video_player.play();
                    self.playing = true;
                }
            },
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
