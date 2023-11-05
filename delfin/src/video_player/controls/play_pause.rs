use std::{
    cell::RefCell,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, MessageBroker, SimpleComponent};

use crate::{tr, video_player::backends::VideoPlayerBackend};

pub struct PlayPauseBroker(RwLock<MessageBroker<PlayPauseInput>>);

impl PlayPauseBroker {
    const fn new() -> Self {
        Self(RwLock::new(MessageBroker::new()))
    }

    pub(crate) fn read(&self) -> RwLockReadGuard<MessageBroker<PlayPauseInput>> {
        self.0.read().unwrap()
    }

    pub(crate) fn reset(&self) {
        *self.0.write().unwrap() = MessageBroker::new();
    }
}

pub static PLAY_PAUSE_BROKER: PlayPauseBroker = PlayPauseBroker::new();

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
            }
        }
    }
}
