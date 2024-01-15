use relm4::adw;
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
enum InnerTheme {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Theme(Option<InnerTheme>);

pub static THEME_LIGHT: Theme = Theme(Some(InnerTheme::Light));
pub static THEME_DARK: Theme = Theme(Some(InnerTheme::Dark));

impl From<Theme> for u32 {
    fn from(value: Theme) -> Self {
        match value.0 {
            None => 0,
            Some(InnerTheme::Light) => 1,
            Some(InnerTheme::Dark) => 2,
        }
    }
}

impl From<u32> for Theme {
    fn from(value: u32) -> Self {
        match value {
            0 => Theme(None),
            1 => THEME_LIGHT,
            2 => THEME_DARK,
            _ => unreachable!("theme index {value} does not exist"),
        }
    }
}

impl From<Theme> for adw::ColorScheme {
    fn from(val: Theme) -> Self {
        match val.0 {
            None => adw::ColorScheme::Default,
            Some(InnerTheme::Light) => adw::ColorScheme::ForceLight,
            Some(InnerTheme::Dark) => adw::ColorScheme::ForceDark,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    pub language: Option<LanguageIdentifier>,
    pub theme: Theme,
}

impl GeneralConfig {
    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        adw::StyleManager::default().set_color_scheme(self.theme.into());
    }
}
