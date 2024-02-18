use chrono::prelude::*;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};

use crate::tr;

pub trait DisplayYears {
    fn display_years(&self) -> Option<String>;
}

// Displays years show was in production.
// Should more or less match Jellyfin's web client behaviour: https://github.com/jellyfin/jellyfin-web/blob/c5520bb5ac72d02e49ac60e47bdb2ac33940c2d5/src/components/mediainfo/mediainfo.js#L177-L197
impl DisplayYears for BaseItemDto {
    fn display_years(&self) -> Option<String> {
        let premiere_year = self.premiere_date.map(|d| d.year());
        let end_year = self.end_date.as_ref().map(|d| d.year());

        if let Some(BaseItemKind::Series) = self.type_ {
            let start_year = self.production_year.as_ref().or(premiere_year.as_ref());

            if let Some("Continuing") = self.status.as_deref() {
                return Some(match start_year {
                    Some(start_year) => {
                        tr!("media-details-years.until-present", {"startYear" => start_year})
                            .to_string()
                    }
                    _ => tr!("media-details-years.present").to_string(),
                });
            }

            if let Some(end_year) = &end_year {
                if let Some(start_year) = start_year {
                    if start_year == end_year {
                        return Some(start_year.to_string());
                    }

                    return Some(
                        tr!("media-details-years", {
                            "startYear" => start_year,
                            "endYear" => end_year,
                        })
                        .to_string(),
                    );
                }
            }
        }

        self.production_year
            .or(premiere_year)
            .or(end_year)
            .map(|y| y.to_string())
    }
}
