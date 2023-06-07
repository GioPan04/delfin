use gst::glib::WeakRef;
use gtk::prelude::*;
use relm4::{gtk, ComponentParts, SimpleComponent};

pub struct Volume {
    player: WeakRef<gstplay::Play>,
    muted: bool,
}

#[derive(Debug)]
pub enum VolumeInput {
    ToggleMute,
}

#[relm4::component(pub)]
impl SimpleComponent for Volume {
    type Init = WeakRef<gstplay::Play>;
    type Input = VolumeInput;
    type Output = ();

    view! {
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
            set_halign: gtk::Align::End,
            set_hexpand: true,
            connect_clicked[sender] => move |_| {
                sender.input(VolumeInput::ToggleMute);
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Volume {
            player: init,
            muted: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            VolumeInput::ToggleMute => {
                if let Some(player) = self.player.upgrade() {
                    self.muted = !self.muted;
                    player.set_mute(self.muted);
                }
            }
        }
    }
}
