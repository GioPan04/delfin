use std::{cell::RefCell, sync::Arc, time::Duration};

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, SimpleComponent};

use crate::{
    globals::CONFIG,
    tr,
    utils::{debounce::Debounce, message_broker::ResettableMessageBroker},
    video_player::backends::VideoPlayerBackend,
};

pub static VOLUME_BROKER: ResettableMessageBroker<VolumeInput> = ResettableMessageBroker::new();

pub struct Volume {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    muted: bool,
    volume: f64,
    save_debounce: Debounce,
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

    SaveSettings,
    LoadSettings,
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
        let config = CONFIG.read();

        let save_debounce = Debounce::new(
            Duration::from_secs(3),
            Box::new({
                let sender = sender.clone();
                move || {
                    sender.input(VolumeInput::SaveSettings);
                }
            }),
        );

        let model = Volume {
            video_player: video_player.clone(),
            muted: config.video_player.muted,
            volume: config.video_player.volume,
            save_debounce,
        };

        let widgets = view_output!();

        model.video_player.borrow_mut().connect_mute_updated({
            let sender = sender.clone();
            Box::new(move |muted| {
                sender.input(VolumeInput::MuteUpdated(muted));
            })
        });

        model
            .video_player
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
                self.save_debounce.debounce();
            }
            VolumeInput::SetVolume(volume) => {
                self.volume = volume;
                self.video_player.borrow().set_volume(volume);
                self.save_debounce.debounce();
            }
            VolumeInput::ChangeVolume(amount) => {
                let volume = f64::clamp(self.volume + amount, 0.0, 1.0);
                self.volume = volume;
                self.video_player.borrow().set_volume(volume);
                self.save_debounce.debounce();
            }
            VolumeInput::SaveSettings => {
                self.save_settings();
            }
            VolumeInput::LoadSettings => {
                let config = CONFIG.read();
                self.volume = config.video_player.volume;
                self.muted = config.video_player.muted;
                self.video_player.borrow().set_muted(self.muted);
                self.video_player.borrow().set_volume(self.volume);
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        self.save_settings();
    }
}

impl Volume {
    fn save_settings(&self) {
        let mut config = CONFIG.write();
        config.video_player.volume = self.volume;
        config.video_player.muted = self.muted;
        config.save().expect("Error saving volume settings")
    }
}
