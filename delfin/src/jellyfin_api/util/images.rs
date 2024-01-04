use anyhow::{bail, Result};
use jellyfin_api::types::BaseItemDto;

use crate::{jellyfin_api::api_client::ApiClient, media_details::episode::EPISODE_THUMBNAIL_SIZE};

// TODO: should actually fetch images, not just URL

impl ApiClient {
    pub fn get_episode_thumbnail_url(&self, item: &BaseItemDto) -> Result<String> {
        let item_id = match item.id {
            Some(item_id) => item_id,
            None => bail!("Missing item ID"),
        };

        let mut url = self.root.join(&format!("Items/{item_id}/Images/Primary"))?;
        url.query_pairs_mut()
            .append_pair("fillHeight", &EPISODE_THUMBNAIL_SIZE.to_string())
            .append_pair("quality", "96");

        Ok(url.to_string())
    }

    pub fn get_parent_or_item_thumbnail_url(&self, item: &BaseItemDto) -> Result<String> {
        let item_id = match item.parent_backdrop_item_id.or(item.id) {
            Some(item_id) => item_id,
            None => bail!("Missing parent backdrop item ID"),
        };

        let mut url = self.root.join(&format!("Items/{item_id}/Images/Primary"))?;
        url.query_pairs_mut()
            .append_pair("fillWidth", "200")
            .append_pair("quality", "96");

        Ok(url.to_string())
    }

    pub fn get_collection_thumbnail_url(&self, item: &BaseItemDto) -> Result<String> {
        let item_id = match item.parent_backdrop_item_id.or(item.id) {
            Some(item_id) => item_id,
            None => bail!("Missing parent backdrop item ID"),
        };

        let mut url = self.root.join(&format!("Items/{item_id}/Images/Primary"))?;
        url.query_pairs_mut()
            .append_pair("fillWidth", "350")
            .append_pair("quality", "96");

        Ok(url.to_string())
    }

    pub fn get_parent_or_item_backdrop_url(&self, item: &BaseItemDto) -> Result<String> {
        let item_id = match item.parent_backdrop_item_id.or(item.id) {
            Some(item_id) => item_id,
            None => bail!("Missing parent backdrop item ID"),
        };

        let mut url = self
            .root
            .join(&format!("Items/{item_id}/Images/Backdrop"))?;
        url.query_pairs_mut()
            .append_pair("fillWidth", "350")
            .append_pair("quality", "96");

        Ok(url.to_string())
    }

    pub fn get_backdrop_url(&self, item: &BaseItemDto) -> Result<String> {
        let item_id = match item.id {
            Some(item_id) => item_id,
            None => bail!("Missing parent backdrop item ID"),
        };

        let mut url = self
            .root
            .join(&format!("Items/{item_id}/Images/Backdrop"))?;
        url.query_pairs_mut()
            .append_pair("maxWidth", "1440")
            .append_pair("quality", "80");

        Ok(url.to_string())
    }

    pub fn get_next_up_thumbnail_url(&self, item: &BaseItemDto) -> Result<String> {
        let item_id = match item.id {
            Some(item_id) => item_id,
            None => bail!("Missing item ID"),
        };

        let mut url = self.root.join(&format!("Items/{item_id}/Images/Primary"))?;
        url.query_pairs_mut()
            .append_pair("fillHeight", "150")
            .append_pair("quality", "96");

        Ok(url.to_string())
    }
}
