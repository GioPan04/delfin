use std::{cell::RefCell, sync::Arc};

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::{
    tr, utils::message_broker::ResettableMessageBroker, video_player::backends::VideoPlayerBackend,
};

pub static PLAY_PAUSE_BROKER: ResettableMessageBroker<PlayPauseInput> =
    ResettableMessageBroker::new();

pub(crate) struct PlayPause {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    loading: bool,
    playing: bool,
}

#[derive(Debug)]
pub enum PlayPauseInput {
    TogglePlaying,
    SetLoading,
    SetPlaying(bool),
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
                    self.video_player.borrow().pause();
                    self.playing = false;
                }
                false => {
                    self.video_player.borrow().play();
                    self.playing = true;
                }
            },
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
        }
    }
}
