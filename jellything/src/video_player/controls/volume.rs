use std::{cell::RefCell, sync::Arc};

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, SimpleComponent};

use crate::video_player::backends::VideoPlayerBackend;

pub struct Volume {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    muted: bool,
    volume: f64,
}

#[derive(Debug)]
pub enum VolumeInput {
    ToggleMute,
    UpdateMute(bool),
    // Set video player volume
    SetVolume(f64),
    // Update displayed volume
    UpdateVolume(f64),
}

#[relm4::component(pub)]
impl SimpleComponent for Volume {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = VolumeInput;
    type Output = ();

    view! {
        gtk::Box {
            gtk::Separator { add_css_class: "spacer" },

            gtk::Button {
                #[watch]
                // TODO: icon is oddly bright
                set_icon_name: if model.muted {
                    "audio-volume-muted"
                } else {
                    "audio-volume-high"
                },
                #[watch]
                set_tooltip_text: Some(if model.muted {
                    "Unmute"
                } else {
                    "Mute"
                }),
                // set_halign: gtk::Align::End,
                // set_hexpand: true,
                connect_clicked[sender] => move |_| {
                    sender.input(VolumeInput::ToggleMute);
                },
            },

            gtk::Scale {
                #[watch]
                #[block_signal(volume_changed_handler)]
                set_value: model.volume,
                set_range: (0.0, 1.0),
                #[watch]
                set_sensitive: !model.muted,
                // TODO: responsive
                set_width_request: 125,
                connect_value_changed[sender] => move |scale| {
                    sender.input(VolumeInput::SetVolume(scale.value()));
                } @volume_changed_handler,
            },

            gtk::Separator { add_css_class: "spacer" },
        }
    }

    fn init(
        video_player: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Volume {
            video_player: video_player.clone(),
            muted: video_player.borrow().muted(),
            volume: video_player.borrow().volume(),
        };

        let widgets = view_output!();

        video_player.borrow_mut().connect_mute_updated({
            let sender = sender.clone();
            Box::new(move |muted| {
                sender.input(VolumeInput::UpdateMute(muted));
            })
        });

        video_player
            .borrow_mut()
            .connect_volume_updated(Box::new(move |volume| {
                sender.input(VolumeInput::UpdateVolume(volume));
            }));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            VolumeInput::ToggleMute => {
                self.muted = !self.muted;
                self.video_player.borrow().set_muted(self.muted);
            }
            VolumeInput::UpdateMute(muted) => self.muted = muted,
            VolumeInput::SetVolume(volume) => {
                self.volume = volume;
                self.video_player.borrow().set_volume(volume);
            }
            VolumeInput::UpdateVolume(volume) => self.volume = volume,
        }
    }
}
