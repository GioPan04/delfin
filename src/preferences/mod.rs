use adw::prelude::*;
use relm4::prelude::*;

use crate::{config::video_player_config::VideoPlayerSkipAmount, globals::CONFIG};

pub struct Preferences;

#[derive(Debug)]
pub enum PreferencesInput {
    SkipBackwardsAmount(u32),
    SkipForwardsAmount(u32),
}

#[relm4::component(pub)]
impl SimpleComponent for Preferences {
    type Init = ();
    type Input = PreferencesInput;
    type Output = ();

    view! {
        adw::PreferencesWindow {
            set_visible: true,
            set_modal: true,
            set_title: Some("Preferences"),

            add = &adw::PreferencesPage {
                set_title: "General",
                add = &adw::PreferencesGroup {
                    set_title: "Video Player",

                    add = &adw::ComboRow {
                        set_title: "Skip backwards amount",
                        set_subtitle: "How many seconds to skip backwards at a time",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&["10 seconds", "30 seconds"]),
                        set_selected: if let VideoPlayerSkipAmount::Ten = video_player_config.skip_backwards_amount { 0 } else { 1 },
                        connect_selected_notify[sender] => move |cb| {
                            sender.input(PreferencesInput::SkipBackwardsAmount(cb.selected()));
                        },
                    },
                    add = &adw::ComboRow {
                        set_title: "Skip forwards amount",
                        set_subtitle: "How many seconds to skip forwards at a time",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&["10 seconds", "30 seconds"]),
                        set_selected: if let VideoPlayerSkipAmount::Ten = video_player_config.skip_forwards_amount { 0 } else { 1 },
                        connect_selected_notify[sender] => move |cb| {
                            sender.input(PreferencesInput::SkipForwardsAmount(cb.selected()));
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = CONFIG.read();
        let video_player_config = &config.video_player;

        let model = Preferences;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PreferencesInput::SkipBackwardsAmount(index) => {
                let mut config = CONFIG.write();
                config.video_player.skip_backwards_amount = match index {
                    0 => VideoPlayerSkipAmount::Ten,
                    _ => VideoPlayerSkipAmount::Thirty,
                };
                config.save().expect("Error saving config");
            }
            PreferencesInput::SkipForwardsAmount(index) => {
                let mut config = CONFIG.write();
                config.video_player.skip_forwards_amount = match index {
                    0 => VideoPlayerSkipAmount::Ten,
                    _ => VideoPlayerSkipAmount::Thirty,
                };
                config.save().expect("Error saving config");
            }
        }
    }
}
