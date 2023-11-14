use std::{
    cell::RefCell,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, MessageBroker, SimpleComponent};

use crate::{tr, video_player::backends::VideoPlayerBackend};

pub struct Volume {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    muted: bool,
    volume: f64,
}

#[derive(Debug)]
pub enum VolumeInput {
    // Update displayed values
    MuteUpdated(bool),
    VolumeUpdated(f64),
    // Modify player volume/mute
    ToggleMute,
    SetVolume(f64),
    ChangeVolume(f64),
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
                set_focus_on_click: false,

                #[watch]
                // TODO: icon is oddly bright
                set_icon_name: if model.muted {
                    "audio-volume-muted"
                } else {
                    "audio-volume-high"
                },
                #[watch]
                set_tooltip_text: Some(tr!(
                    "vp-volume-mute-tooltip",
                    {"muted" => model.muted.to_string()},
                )),
                // set_halign: gtk::Align::End,
                // set_hexpand: true,
                connect_clicked[sender] => move |_| {
                    sender.input(VolumeInput::ToggleMute);
                },
            },

            gtk::Scale {
                set_focus_on_click: false,

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
                sender.input(VolumeInput::MuteUpdated(muted));
            })
        });

        video_player
            .borrow_mut()
            .connect_volume_updated(Box::new(move |volume| {
                sender.input(VolumeInput::VolumeUpdated(volume));
            }));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            VolumeInput::MuteUpdated(muted) => self.muted = muted,
            VolumeInput::VolumeUpdated(volume) => self.volume = volume,
            VolumeInput::ToggleMute => {
                self.muted = !self.muted;
                self.video_player.borrow().set_muted(self.muted);
            }
            VolumeInput::SetVolume(volume) => {
                self.volume = volume;
                self.video_player.borrow().set_volume(volume);
            }
            VolumeInput::ChangeVolume(amount) => {
                let volume = f64::clamp(self.volume + amount, 0.0, 1.0);
                self.volume = volume;
                self.video_player.borrow().set_volume(volume);
            }
        }
    }
}

pub struct VolumeBroker(RwLock<MessageBroker<VolumeInput>>);

impl VolumeBroker {
    const fn new() -> Self {
        Self(RwLock::new(MessageBroker::new()))
    }

    pub fn read(&self) -> RwLockReadGuard<MessageBroker<VolumeInput>> {
        self.0.read().unwrap()
    }

    pub fn reset(&self) {
        *self.0.write().unwrap() = MessageBroker::new();
    }
}

pub static VOLUME_BROKER: VolumeBroker = VolumeBroker::new();
