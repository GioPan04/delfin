use gst::glib::WeakRef;
use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PlayPause {
    player: WeakRef<gstplay::Play>,
    playing: bool,
}

#[derive(Debug)]
pub enum PlayPauseInput {
    TogglePlaying,
}

#[relm4::component(pub)]
impl SimpleComponent for PlayPause {
    type Init = WeakRef<gstplay::Play>;
    type Input = PlayPauseInput;
    type Output = ();

    view! {
        gtk::Button {
            #[watch]
            set_icon_name: if model.playing {
                "media-playback-pause"
            } else {
                "media-playback-start"
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
            player: init,
            playing: true,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PlayPauseInput::TogglePlaying => {
                if let Some(player) = self.player.upgrade() {
                    match self.playing {
                        true => {
                            player.pause();
                            self.playing = false;
                        }
                        false => {
                            player.play();
                            self.playing = true;
                        }
                    }
                }
            }
        }
    }
}
