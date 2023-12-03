use adw::prelude::*;
use gtk::gdk;
use relm4::prelude::*;

use crate::config::video_player_config::VideoPlayerConfig;
use crate::globals::CONFIG;
use crate::tr;
use crate::utils::rgba::RGBA;

pub(crate) struct SubtitlesPreferences {
    video_player_config: VideoPlayerConfig,
}

#[derive(Debug)]
pub(crate) enum SubtitlesPreferencesInput {
    UpdateConfig(VideoPlayerConfig),
    Reset,
    SubtitleScale(f64),
    SubtitleColour(RGBA),
    SubtitleBackgroundColour(RGBA),
    SubtitlePosition(f64),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for SubtitlesPreferences {
    type Init = ();
    type Input = SubtitlesPreferencesInput;
    type Output = ();

    view! {
        adw::PreferencesGroup {
            set_title: tr!("prefs-vp-subs"),

            #[wrap(Some)]
            set_header_suffix = &gtk::Button {
                adw::ButtonContent {
                    set_icon_name: "edit-undo",
                    set_label: tr!("prefs-vp-subs-reset.label"),
                },
                set_tooltip: tr!("prefs-vp-subs-reset.tooltip"),
                add_css_class: "flat",
                #[watch]
                set_sensitive: model.can_reset(),
                connect_clicked[sender] => move |_| {
                    sender.input(SubtitlesPreferencesInput::Reset);
                },
            },

            add = &adw::SpinRow::new(
                Some(&gtk::Adjustment::new(
                    model.video_player_config.subtitle_scale,
                    0.0, 100.0, 0.1, 1.0, 0.0,
                )),
                // Climb rate
                1.0,
                // Digits
                1,
            ) {
                set_title: tr!("prefs-vp-subs-scale.title"),
                set_subtitle: tr!("prefs-vp-subs-scale.subtitle"),

                #[watch]
                #[block_signal(subtitle_scale_change_handler)]
                set_value: model.video_player_config.subtitle_scale,
                connect_changed[sender] => move |spinrow| {
                    sender.input(SubtitlesPreferencesInput::SubtitleScale(spinrow.value()));
                } @subtitle_scale_change_handler,
            },

            add = &adw::ActionRow {
                set_title: tr!("prefs-vp-subs-colour.title"),
                add_suffix = &gtk::Label {
                    #[watch]
                    set_label: &model.video_player_config.subtitle_colour,
                },
                add_suffix = &gtk::ColorDialogButton {
                    set_valign: gtk::Align::Center,
                    set_dialog = &gtk::ColorDialog,
                    #[watch]
                    #[block_signal(colour_change_handler)]
                    set_rgba: &gdk::RGBA::parse(model.video_player_config.subtitle_colour.clone())
                        .unwrap_or_else(|_| panic!("Error parsing colour: {}", model.video_player_config.subtitle_colour.clone())),
                    connect_rgba_notify[sender] => move |btn| {
                        sender.input(SubtitlesPreferencesInput::SubtitleColour(btn.rgba().into()));
                    } @colour_change_handler,
                },
            },

            add = &adw::ActionRow {
                set_title: tr!("prefs-vp-subs-background-colour.title"),
                add_suffix = &gtk::Label {
                    #[watch]
                    set_label: &model.video_player_config.subtitle_background_colour,
                },
                add_suffix = &gtk::ColorDialogButton {
                    set_valign: gtk::Align::Center,
                    set_dialog = &gtk::ColorDialog,
                    #[watch]
                    #[block_signal(background_colour_change_handler)]
                    set_rgba: &gdk::RGBA::parse(model.video_player_config.subtitle_background_colour.clone())
                        .unwrap_or_else(|_| panic!("Error parsing colour: {}", model.video_player_config.subtitle_background_colour.clone())),
                    connect_rgba_notify[sender] => move |btn| {
                        sender.input(SubtitlesPreferencesInput::SubtitleBackgroundColour(btn.rgba().into()));
                    } @background_colour_change_handler,
                },
            },

            add = &adw::SpinRow::new(
                Some(&gtk::Adjustment::new(
                    model.video_player_config.subtitle_position as f64,
                    0.0, 150.0, 1.0, 1.0, 0.0,
                )),
                // Climb rate
                1.0,
                // Digits
                0,
            ) {
                set_title: "Subtitles position",
                set_subtitle: "Position on screen, where 0 is the top of the screen, and 100 is the bottom",

                #[watch]
                #[block_signal(subtitle_position_change_handler)]
                set_value: model.video_player_config.subtitle_position as f64,
                connect_changed[sender] => move |spinrow| {
                    sender.input(SubtitlesPreferencesInput::SubtitlePosition(spinrow.value()));
                } @subtitle_position_change_handler,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SubtitlesPreferences {
            video_player_config: CONFIG.read().video_player.clone(),
        };
        CONFIG.subscribe(sender.input_sender(), |config| {
            SubtitlesPreferencesInput::UpdateConfig(config.video_player.clone())
        });

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        if let SubtitlesPreferencesInput::UpdateConfig(video_player_config) = message {
            self.video_player_config = video_player_config;
            return;
        }

        let mut config = CONFIG.write();

        match message {
            SubtitlesPreferencesInput::UpdateConfig(_) => {
                // Already handled above
                unreachable!();
            }
            SubtitlesPreferencesInput::Reset => {
                let default = VideoPlayerConfig::default();
                config.video_player.subtitle_scale = default.subtitle_scale;
                config.video_player.subtitle_colour = default.subtitle_colour;
                config.video_player.subtitle_background_colour = default.subtitle_background_colour;
                config.video_player.subtitle_position = default.subtitle_position;
            }
            SubtitlesPreferencesInput::SubtitleScale(subtitle_scale) => {
                config.video_player.subtitle_scale = subtitle_scale;
            }
            SubtitlesPreferencesInput::SubtitleColour(colour) => {
                config.video_player.subtitle_colour = colour.to_hex();
            }
            SubtitlesPreferencesInput::SubtitleBackgroundColour(background_colour) => {
                config.video_player.subtitle_background_colour = background_colour.to_hex();
            }
            SubtitlesPreferencesInput::SubtitlePosition(position) => {
                config.video_player.subtitle_position = position as u32;
            }
        }

        config.save().expect("Error saving config");
    }
}

impl SubtitlesPreferences {
    fn can_reset(&self) -> bool {
        let default = VideoPlayerConfig::default();
        let video_player_config = &self.video_player_config;

        (video_player_config.subtitle_scale != default.subtitle_scale)
            || (video_player_config.subtitle_colour != default.subtitle_colour)
            || (video_player_config.subtitle_background_colour
                != default.subtitle_background_colour)
            || (video_player_config.subtitle_position != default.subtitle_position)
    }
}
