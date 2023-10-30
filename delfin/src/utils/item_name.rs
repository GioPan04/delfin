use jellyfin_api::types::BaseItemDto;

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
            return Some(format!("S{parent_index_number}:E{index_number} - {name}"));
        }

        Some(name.to_string())
    }
}
