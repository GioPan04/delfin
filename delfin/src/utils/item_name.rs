use jellyfin_api::types::BaseItemDto;

use crate::tr;

pub trait ItemName {
    fn episode_name_with_number(&self) -> Option<String>;
}

impl ItemName for BaseItemDto {
    fn episode_name_with_number(&self) -> Option<String> {
        let name = match &self.name {
            Some(name) => name,
            _ => {
                return None;
            }
        };

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
