use libadwaita as adw;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Default,
    Light,
    Dark,
}

impl From<Theme> for adw::ColorScheme {
    fn from(val: Theme) -> Self {
        match val {
            Theme::Default => adw::ColorScheme::Default,
            Theme::Light => adw::ColorScheme::ForceLight,
            Theme::Dark => adw::ColorScheme::ForceDark,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default)]
    theme: Theme,
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
