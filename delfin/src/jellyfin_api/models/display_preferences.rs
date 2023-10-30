use std::{collections::HashMap, fmt::Display, str::FromStr};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayPreferencesRaw {
    pub custom_prefs: Option<HashMap<String, Option<String>>>,
}

#[derive(Clone, Debug)]
pub enum HomeSection {
    MyMedia,
    MyMediaSmall,
    ActiveRecordings,
    ContinueWatching,
    ContinueListening,
    ContinueReading,
    LatestMedia,
    NextUp,
    LiveTV,
    None,
}

impl Display for HomeSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MyMedia => "smalllibrarytiles",
                Self::MyMediaSmall => "librarybuttons",
                Self::ActiveRecordings => "activerecordings",
                Self::ContinueWatching => "resume",
                Self::ContinueListening => "resumeaudio",
                Self::ContinueReading => "resumebook",
                Self::LatestMedia => "latestmedia",
                Self::NextUp => "nextup",
                Self::LiveTV => "livetv",
                Self::None => "none",
            }
        )
    }
}

#[derive(Debug)]
pub struct ParseHomeSectionError;

impl FromStr for HomeSection {
    type Err = ParseHomeSectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "smalllibrarytiles" => Ok(Self::MyMedia),
            "librarybuttons" => Ok(Self::MyMediaSmall),
            "activerecordings" => Ok(Self::ActiveRecordings),
            "resume" => Ok(Self::ContinueWatching),
            "resumeaudio" => Ok(Self::ContinueListening),
            "resumebook" => Ok(Self::ContinueReading),
            "latestmedia" => Ok(Self::LatestMedia),
            "nextup" => Ok(Self::NextUp),
            "livetv" => Ok(Self::LiveTV),
            "none" => Ok(Self::None),
            _ => Err(ParseHomeSectionError),
        }
    }
}

struct HomeSections(Vec<HomeSection>);

impl Default for HomeSections {
    fn default() -> Self {
        // Default sections from Jellyfin's official web client
        // https://github.com/jellyfin/jellyfin-web/blob/b13b1ff76d71381bce0d74c1d0ac78d919acaab0/src/components/homesections/homesections.js#L16-L37
        Self(vec![
            HomeSection::MyMedia,
            HomeSection::ContinueWatching,
            HomeSection::ContinueListening,
            HomeSection::ContinueReading,
            HomeSection::LiveTV,
            HomeSection::NextUp,
            HomeSection::LatestMedia,
            HomeSection::None,
        ])
    }
}

#[derive(Clone, Debug)]
pub struct DisplayPreferences {
    pub home_sections: Vec<HomeSection>,
}

impl From<DisplayPreferencesRaw> for DisplayPreferences {
    fn from(value: DisplayPreferencesRaw) -> Self {
        let mut home_sections = HomeSections::default().0;

        // Overwrite default home sections if the user has changed them
        if let Some(custom_prefs) = value.custom_prefs {
            home_sections = home_sections
                .into_iter()
                .enumerate()
                .map(|(n, default)| {
                    // TODO: ideally the custom_prefs map would skip over null
                    // values so that we don't have nested Options
                    if let Some(Some(val)) = custom_prefs.get(&format!("homesection{n}")) {
                        if let Ok(home_section) = HomeSection::from_str(val) {
                            return home_section;
                        }
                    }
                    default
                })
                .collect();
        }

        DisplayPreferences { home_sections }
    }
}
