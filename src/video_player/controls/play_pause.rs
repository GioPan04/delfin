use std::cell::OnceCell;

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::video_player::gst_play_widget::GstVideoPlayer;

pub struct PlayPause {
    video_player: OnceCell<GstVideoPlayer>,
    playing: bool,
}

#[derive(Debug)]
pub enum PlayPauseInput {
    TogglePlaying,
}

#[relm4::component(pub)]
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
        }
    }
}
