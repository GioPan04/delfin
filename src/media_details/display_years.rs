use crate::jellyfin_api::api::item::{GetItemRes, ItemType};

pub(crate) trait DisplayYears {
    fn display_years(&self) -> Option<String>;
}

// Displays years show was in production.
// Should more or less match Jellyfin's web client behaviour: https://github.com/jellyfin/jellyfin-web/blob/c5520bb5ac72d02e49ac60e47bdb2ac33940c2d5/src/components/mediainfo/mediainfo.js#L177-L197
impl DisplayYears for GetItemRes {
    fn display_years(&self) -> Option<String> {
        let production_year = self.production_year.map(|y| y.to_string());
        let premiere_year = self.premiere_date.as_ref().map(|d| d.date.year.to_string());
        let end_year = self.end_date.as_ref().map(|d| d.date.year.to_string());

        if let ItemType::Series = self.item_type {
            let start_year = production_year.as_ref().or(premiere_year.as_ref());

            if let Some("Continuing") = self.status.as_deref() {
                return Some(match start_year {
                    Some(start_year) => format!("{start_year} – Present"),
                    _ => "Present".to_string(),
                });
            }

            if let Some(end_year) = &end_year {
                if let Some(start_year) = start_year {
                    if start_year == end_year {
                        return Some(start_year.clone());
                    }

                    return Some(format!("{start_year} – {end_year}"));
                }
            }
        }

        production_year.or(premiere_year).or(end_year)
    }
}
