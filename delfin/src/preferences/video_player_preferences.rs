use adw::prelude::*;
use relm4::prelude::*;

use crate::{
    config::video_player_config::{VideoPlayerBackendPreference, VideoPlayerSkipAmount},
    globals::CONFIG,
    tr,
};

pub struct VideoPlayerPreferences;

#[derive(Debug)]
pub enum VideoPlayerPreferencesInput {
    SkipBackwardsAmount(u32),
    SkipForwardsAmount(u32),
    IntroSkipper(bool),
    Jellyscrub(bool),
    Backend(u32),
    HlsPlayback(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for VideoPlayerPreferences {
    type Init = ();
    type Input = VideoPlayerPreferencesInput;
    type Output = ();

    view! {
        &adw::PreferencesPage {
            set_title: tr!("prefs-vp-page"),
            set_icon_name: Some("play-filled"),

            add = &adw::PreferencesGroup {
                set_title: tr!("prefs-vp-interface"),

                add = &adw::ComboRow {
                    set_title: tr!("prefs-vp-skip-backwards.title"),
                    set_subtitle: tr!("prefs-vp-skip-backwards.subtitle"),
                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        tr!("prefs-skip-amount", {"seconds" => 10}),
                        tr!("prefs-skip-amount", {"seconds" => 30}),
                    ]),
                    set_selected: if let VideoPlayerSkipAmount::Ten = video_player_config.skip_backwards_amount { 0 } else { 1 },
                    connect_selected_notify[sender] => move |cb| {
                        sender.input(VideoPlayerPreferencesInput::SkipBackwardsAmount(cb.selected()));
                    },
                },

                add = &adw::ComboRow {
                    set_title: tr!("prefs-vp-skip-forwards.title"),
                    set_subtitle: tr!("prefs-vp-skip-forwards.subtitle"),
                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        tr!("prefs-skip-amount", {"seconds" => 10}),
                        tr!("prefs-skip-amount", {"seconds" => 30}),
                    ]),
                    set_selected: if let VideoPlayerSkipAmount::Ten = video_player_config.skip_forwards_amount { 0 } else { 1 },
                    connect_selected_notify[sender] => move |cb| {
                        sender.input(VideoPlayerPreferencesInput::SkipForwardsAmount(cb.selected()));
                    },
                },
            },

            add = &adw::PreferencesGroup {
                set_title: tr!("prefs-vp-plugins"),

                add = &adw::SwitchRow {
                    set_title: tr!("prefs-vp-intro-skipper.title"),
                    set_subtitle: tr!("prefs-vp-intro-skipper.subtitle", {
                        "introSkipperUrl" => "https://github.com/ConfusedPolarBear/intro-skipper/",
                    }),
                    set_active: video_player_config.intro_skipper,
                    connect_active_notify[sender] => move |sr| {
                        sender.input(VideoPlayerPreferencesInput::IntroSkipper(sr.is_active()));
                    },
                },

                add = &adw::SwitchRow {
                    set_title: tr!("prefs-vp-jellyscrub.title"),
                    set_subtitle: tr!("prefs-vp-jellyscrub.subtitle", {
                        "jellyscrubUrl" => "https://github.com/nicknsy/jellyscrub/",
                    }),
                    set_active: video_player_config.jellyscrub,
                    connect_active_notify[sender] => move |sr| {
                        sender.input(VideoPlayerPreferencesInput::Jellyscrub(sr.is_active()));
                    },
                },
            },

            add = &adw::PreferencesGroup {
                set_title: tr!("prefs-vp-other"),

                add = &adw::ExpanderRow {
                    set_title: tr!("prefs-vp-experimental.title"),
                    set_subtitle: tr!("prefs-vp-experimental.subtitle"),

                    add_row = &adw::ComboRow {
                        set_visible: cfg!(feature = "gst"),
                        set_title: tr!("prefs-vp-backend.title"),
                        set_subtitle: tr!("prefs-vp-backend.subtitle"),
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            tr!("prefs-vp-backend.value-mpv"),
                            tr!("prefs-vp-backend.value-gstreamer"),
                        ]),
                        set_selected: if let VideoPlayerBackendPreference::Mpv = video_player_config.backend { 0 } else { 1 },
                        connect_selected_notify[sender] => move |cb| {
                            sender.input(VideoPlayerPreferencesInput::Backend(cb.selected()));
                        },
                    },

                    add_row = &adw::SwitchRow {
                        set_title: tr!("prefs-vp-hls-playback"),
                        set_active: video_player_config.hls_playback,
                        connect_active_notify[sender] => move |sr| {
                            sender.input(VideoPlayerPreferencesInput::HlsPlayback(sr.is_active()));
                        },
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = CONFIG.read();
        let video_player_config = &config.video_player;

        let model = VideoPlayerPreferences;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        let mut config = CONFIG.write();

        match message {
            VideoPlayerPreferencesInput::SkipBackwardsAmount(index) => {
                config.video_player.skip_backwards_amount = match index {
                    0 => VideoPlayerSkipAmount::Ten,
                    _ => VideoPlayerSkipAmount::Thirty,
                };
            }
            VideoPlayerPreferencesInput::SkipForwardsAmount(index) => {
                config.video_player.skip_forwards_amount = match index {
                    0 => VideoPlayerSkipAmount::Ten,
                    _ => VideoPlayerSkipAmount::Thirty,
                };
            }
            VideoPlayerPreferencesInput::IntroSkipper(intro_skipper) => {
                config.video_player.intro_skipper = intro_skipper;
            }
            VideoPlayerPreferencesInput::Jellyscrub(jellyscrub) => {
                config.video_player.jellyscrub = jellyscrub;
            }
            VideoPlayerPreferencesInput::Backend(index) => {
                config.video_player.backend = match index {
                    0 => VideoPlayerBackendPreference::Mpv,
                    _ => VideoPlayerBackendPreference::Gst,
                };
            }
            VideoPlayerPreferencesInput::HlsPlayback(active) => {
                config.video_player.hls_playback = active;
            }
        };

        config.save().expect("Error saving config");
    }
}
