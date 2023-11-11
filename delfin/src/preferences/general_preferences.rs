use adw::prelude::*;
use relm4::prelude::*;

use crate::{config::general::Theme, globals::CONFIG, tr};

pub struct GeneralPreferences;

#[derive(Debug)]
pub enum GeneralPreferencesInput {
    ThemeChanged(u32),
}

#[relm4::component(pub)]
impl SimpleComponent for GeneralPreferences {
    type Init = ();
    type Input = GeneralPreferencesInput;
    type Output = ();

    view! {
        &adw::PreferencesPage {
            set_title: tr!("prefs-general-page"),
            set_icon_name: Some("settings"),

            add = &adw::PreferencesGroup {
                add = &adw::ComboRow {
                    set_title: tr!("prefs-general-theme.title"),
                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        tr!("prefs-general-theme.option-default"),
                        tr!("prefs-general-theme.option-light"),
                        tr!("prefs-general-theme.option-dark"),
                    ]),
                    set_selected: general_preferences.theme().into(),
                    connect_selected_notify[sender] => move |cb| {
                        sender.input(GeneralPreferencesInput::ThemeChanged(cb.selected()));
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let config = CONFIG.read();
        let general_preferences = &config.general;

        let model = GeneralPreferences;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        let mut config = CONFIG.write();

        match message {
            GeneralPreferencesInput::ThemeChanged(theme) => {
                config.general.set_theme(Theme::from(theme));
            }
        }

        config.save().expect("Error saving config");
    }
}

impl From<Theme> for u32 {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Default => 0,
            Theme::Light => 1,
            Theme::Dark => 2,
        }
    }
}

impl From<u32> for Theme {
    fn from(value: u32) -> Self {
        match value {
            0 => Theme::Default,
            1 => Theme::Light,
            2 => Theme::Dark,
            _ => unreachable!(),
        }
    }
}
