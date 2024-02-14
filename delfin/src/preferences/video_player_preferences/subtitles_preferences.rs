use adw::prelude::*;
use gtk::{gdk, CustomFilter};
use relm4::prelude::*;

use crate::config::video_player_config::VideoPlayerConfig;
use crate::globals::CONFIG;
use crate::tr;
use crate::utils::rgba::RGBA;
use crate::video_player::backends::VideoPlayerSubtitleFont;

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
    SubtitleFont(VideoPlayerSubtitleFont),
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

            add = &adw::ExpanderRow {
                set_title: tr!("prefs-vp-subs-style"),

                add_row = &adw::ActionRow {
                    set_subtitle: tr!("prefs-vp-subs-style-ass-warning"),
                },

                add_row = &adw::ActionRow {
                    set_title: tr!("prefs-vp-subs-colour.title"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_label: &model.video_player_config.subtitles.colour,
                    },

                    add_suffix = &gtk::ColorDialogButton {
                        set_valign: gtk::Align::Center,
                        set_dialog = &gtk::ColorDialog,
                        #[watch]
                        #[block_signal(colour_change_handler)]
                        set_rgba: &gdk::RGBA::parse(model.video_player_config.subtitles.colour.clone())
                            .unwrap_or_else(|_| panic!("Error parsing colour: {}", model.video_player_config.subtitles.colour.clone())),
                        connect_rgba_notify[sender] => move |btn| {
                            sender.input(SubtitlesPreferencesInput::SubtitleColour(btn.rgba().into()));
                        } @colour_change_handler,
                    },
                },

                add_row = &adw::ActionRow {
                    set_title: tr!("prefs-vp-subs-background-colour.title"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_label: &model.video_player_config.subtitles.background_colour,
                    },

                    add_suffix = &gtk::ColorDialogButton {
                        set_valign: gtk::Align::Center,
                        set_dialog = &gtk::ColorDialog,
                        #[watch]
                        #[block_signal(background_colour_change_handler)]
                        set_rgba: &gdk::RGBA::parse(model.video_player_config.subtitles.background_colour.clone())
                            .unwrap_or_else(|_| panic!("Error parsing colour: {}", model.video_player_config.subtitles.background_colour.clone())),
                        connect_rgba_notify[sender] => move |btn| {
                            sender.input(SubtitlesPreferencesInput::SubtitleBackgroundColour(btn.rgba().into()));
                        } @background_colour_change_handler,
                    },
                },

                add_row = &adw::ActionRow {
                    set_title: tr!("prefs-vp-subs-font.title"),
                    set_subtitle: tr!("prefs-vp-subs-font.subtitle"),

                    add_suffix = &gtk::FontDialogButton {
                        set_valign: gtk::Align::Center,

                        #[watch]
                        #[block_signal(subtitle_font_change_handler)]
                        set_font_desc: &model.video_player_config.subtitles.font.clone().into(),
                        set_use_font: true,
                        set_dialog = &gtk::FontDialog {
                            set_filter: Some(&model.font_filter()),
                        },

                        connect_font_desc_notify[sender] => move |fdb| {
                            let font = fdb.font_desc().unwrap();

                            let mut size = font.size();
                            if !font.is_size_absolute() {
                                size /= 1024;
                            }

                            if let Some(family) = font.family() {
                                sender.input(SubtitlesPreferencesInput::SubtitleFont(VideoPlayerSubtitleFont {
                                    family: family.into(),
                                    size: size as usize,
                                    bold: font.weight() == gtk::pango::Weight::Bold,
                                    italic: font.style() == gtk::pango::Style::Italic,
                                }));
                            }
                        } @subtitle_font_change_handler,
                    },
                },
            },

            add = &adw::ExpanderRow {
                set_title: tr!("prefs-vp-subs-more"),

                add_row = &adw::ActionRow {
                    add_prefix = &gtk::Image::from_icon_name("warning"),
                    set_title: tr!("prefs-vp-subs-more-ass-warning.title"),
                    set_subtitle: tr!("prefs-vp-subs-more-ass-warning.subtitle"),
                },

                add_row = &adw::SpinRow::new(
                    Some(&gtk::Adjustment::new(
                        model.video_player_config.subtitles.scale,
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
                    set_value: model.video_player_config.subtitles.scale,
                    connect_changed[sender] => move |spinrow| {
                        sender.input(SubtitlesPreferencesInput::SubtitleScale(spinrow.value()));
                    } @subtitle_scale_change_handler,
                },


                add_row = &adw::SpinRow::new(
                    Some(&gtk::Adjustment::new(
                        model.video_player_config.subtitles.position as f64,
                        0.0, 150.0, 1.0, 1.0, 0.0,
                    )),
                    // Climb rate
                    1.0,
                    // Digits
                    0,
                ) {
                    set_title: tr!("prefs-vp-subs-position.title"),
                    set_subtitle: tr!("prefs-vp-subs-position.subtitle"),

                    #[watch]
                    #[block_signal(subtitle_position_change_handler)]
                    set_value: model.video_player_config.subtitles.position as f64,
                    connect_changed[sender] => move |spinrow| {
                        sender.input(SubtitlesPreferencesInput::SubtitlePosition(spinrow.value()));
                    } @subtitle_position_change_handler,
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
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
                config.video_player.subtitles.scale = default.subtitles.scale;
                config.video_player.subtitles.colour = default.subtitles.colour;
                config.video_player.subtitles.background_colour =
                    default.subtitles.background_colour;
                config.video_player.subtitles.position = default.subtitles.position;
                config.video_player.subtitles.font = default.subtitles.font;
            }
            SubtitlesPreferencesInput::SubtitleScale(scale) => {
                config.video_player.subtitles.scale = scale;
            }
            SubtitlesPreferencesInput::SubtitleColour(colour) => {
                config.video_player.subtitles.colour = colour.to_hex();
            }
            SubtitlesPreferencesInput::SubtitleBackgroundColour(background_colour) => {
                config.video_player.subtitles.background_colour = background_colour.to_hex();
            }
            SubtitlesPreferencesInput::SubtitlePosition(position) => {
                config.video_player.subtitles.position = position as u32;
            }
            SubtitlesPreferencesInput::SubtitleFont(font) => {
                config.video_player.subtitles.font = font;
            }
        }

        config.save().expect("Error saving config");
    }
}

impl SubtitlesPreferences {
    fn can_reset(&self) -> bool {
        let default = VideoPlayerConfig::default();
        let video_player_config = &self.video_player_config;

        (video_player_config.subtitles.scale != default.subtitles.scale)
            || (video_player_config.subtitles.colour != default.subtitles.colour)
            || (video_player_config.subtitles.background_colour
                != default.subtitles.background_colour)
            || (video_player_config.subtitles.position != default.subtitles.position)
            || (video_player_config.subtitles.font != default.subtitles.font)
    }

    fn font_filter(&self) -> CustomFilter {
        use gtk::pango::{FontFace, Style, Weight};

        CustomFilter::new(|arg| {
            if let Some(font_face) = arg.downcast_ref::<FontFace>() {
                let desc = font_face.describe();

                // MPV only supports normal or bold weight
                if !matches!(desc.weight(), Weight::Normal | Weight::Bold) {
                    return false;
                }

                // MPV only supports normal or italic
                if !matches!(desc.style(), Style::Normal | Style::Italic) {
                    return false;
                }

                return true;
            }

            false
        })
    }
}
