use jellyfin_api::types::BaseItemDto;

use crate::tr;

pub trait ItemName {
    fn series_and_episode(&self) -> Option<String>;
    fn episode_name_with_number(&self) -> Option<String>;
}

impl ItemName for BaseItemDto {
    fn series_and_episode(&self) -> Option<String> {
        match (&self.series_name, self.episode_name_with_number()) {
            (Some(series_name), Some(episode_name)) => Some(
                tr!("library-series-and-episode-name", {
                    "seriesName" => series_name.to_string(),
                    "episodeName" => episode_name,
                })
                .to_string(),
            ),
            (_, Some(episode_name)) => Some(episode_name),
            _ => None,
        }
    }

    fn episode_name_with_number(&self) -> Option<String> {
        let name = self.name.as_ref()?;

        if let (Some(index_number), Some(parent_index_number)) =
            (self.index_number, self.parent_index_number)
        {
            return Some(
                tr!("library-episode-name-with-season-and-episode", {
                    "seasonNumber" => parent_index_number,
                    "episodeNumber" => index_number,
                    "episodeName" => name.to_string(),
                })
                .to_string(),
            );
        }

        Some(name.to_string())
    }
}
