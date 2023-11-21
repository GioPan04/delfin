use std::mem::take;

use glib::SignalHandlerId;
use gtk::{glib, prelude::*};
use relm4::{gtk, ComponentParts, SimpleComponent};

use crate::{tr, utils::main_window::get_main_window};

use super::control_broker::ControlBroker;

pub static FULLSCREEN_BROKER: ControlBroker<FullscreenInput> = ControlBroker::new();

pub struct Fullscreen {
    fullscreen: bool,
    fullscreened_signal_handler_id: Option<SignalHandlerId>,
}

#[derive(Debug)]
pub enum FullscreenInput {
    ToggleFullscreen,
    WindowFullscreenChanged(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Fullscreen {
    type Init = ();
    type Input = FullscreenInput;
    type Output = ();

    view! {
        gtk::Button {
            set_focus_on_click: false,

            #[watch]
            // TODO: probably find better icons
            set_icon_name: if model.fullscreen {
                "view-restore"
            } else {
                "view-fullscreen"
            },
            #[watch]
            set_tooltip_text: Some(tr!(
                "vp-fullscreen-tooltip",
                {"enter" => (!model.fullscreen).to_string()},
            )),
            connect_clicked[sender] => move |_| {
                sender.input(FullscreenInput::ToggleFullscreen);
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let window = get_main_window().expect("Failed to get main window.");

        let fullscreened_signal_handler_id = window.connect_notify(Some("fullscreened"), {
            let sender = sender.clone();
            move |window, _| {
                sender.input(FullscreenInput::WindowFullscreenChanged(
                    window.is_fullscreen(),
                ));
            }
        });

        let model = Fullscreen {
            fullscreen: window.is_fullscreen(),
            fullscreened_signal_handler_id: Some(fullscreened_signal_handler_id),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        if let Some(window) = get_main_window() {
            let fullscreened_signal_handler_id = take(&mut self.fullscreened_signal_handler_id);
            window.disconnect(fullscreened_signal_handler_id.unwrap());
        }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            FullscreenInput::ToggleFullscreen => {
                self.fullscreen = !self.fullscreen;
                if let Some(window) = get_main_window() {
                    window.set_fullscreened(self.fullscreen);
                }
            }
            FullscreenInput::WindowFullscreenChanged(fullscreen) => {
                self.fullscreen = fullscreen;
            }
        }
    }
}
