use std::{str::FromStr, unreachable};

use adw::prelude::*;
use fluent_templates::Loader;
use relm4::prelude::*;
use unic_langid::LanguageIdentifier;

use crate::{
    config::{general::Theme, Config},
    globals::CONFIG,
    locales::{DEFAULT_LANGUAGE, LOCALES},
    tr,
};

pub struct GeneralPreferences;

#[derive(Debug)]
pub enum GeneralPreferencesInput {
    Language(Option<String>),
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
                #[name = "language"]
                add = &adw::ComboRow {
                    set_title: tr!("prefs-general-language.title"),
                    set_subtitle: tr!("prefs-general-language.subtitle", {
                        "weblateUrl" => "https://translate.codeberg.org/engage/delfin/",
                    }),
                    set_use_markup: true,

                    set_model: Some(&{
                        let sl = gtk::StringList::from_iter(LOCALES
                            .locales()
                            .map(|l| l.to_string())
                        );
                        // Add system default option to top
                        sl.splice(0, 0, &[&tr!("prefs-general-language.option-default", {
                            "languageId" => *DEFAULT_LANGUAGE.to_string(),
                        })]);
                        sl
                    }),
                    set_selected: get_selected_language(&language, config.clone()),

                    connect_selected_notify[sender] => move |cb| {
                        match (cb.selected(), cb.selected_item()) {
                            // First item is system default
                            (0, _) => sender.input(GeneralPreferencesInput::Language(None)),
                            (_, Some(item)) => {
                                if let Ok(lang) = item.downcast::<gtk::StringObject>() {
                                    let lang = lang.string();
                                    sender.input(GeneralPreferencesInput::Language(Some(lang.into())));
                                }
                            }
                            _ => unreachable!(),
                        };
                    },
                },

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
            GeneralPreferencesInput::Language(language) => {
                config.general.language = language
                    .map(|l| LanguageIdentifier::from_str(&l).expect("Error setting language"));
            }
            GeneralPreferencesInput::ThemeChanged(theme) => {
                config.general.set_theme(Theme::from(theme));
            }
        }

        config.save().expect("Error saving config");
    }
}

fn get_selected_language(language: &adw::ComboRow, config: Config) -> u32 {
    match config.general.language {
        // First item is system default
        None => 0,
        Some(selected_language) => {
            let model = language
                .model()
                .unwrap()
                .downcast::<gtk::StringList>()
                .unwrap();
            for (i, lang) in model.snapshot().into_iter().enumerate() {
                let lang = lang
                    .downcast::<gtk::StringObject>()
                    .unwrap()
                    .string()
                    .to_string();
                if lang == selected_language.to_string() {
                    return i as u32;
                }
            }
            unreachable!();
        }
    }
}
