use anyhow::{bail, Result};
use jellyfin_api::types::BaseItemDto;

use crate::jellyfin_api::api_client::ApiClient;

// TODO: should actually fetch images, not just URL

impl ApiClient {
    pub fn get_episode_primary_image_url(&self, item: &BaseItemDto, height: i32) -> Result<String> {
        let item_id = match item.id {
            Some(item_id) => item_id,
            None => bail!("Missing item ID"),
        };

        let images_tags = match item.image_tags.as_ref() {
            Some(tags) => tags,
            None => bail!("Missing images tags"),
        };

        let image_tag = match images_tags.get("Primary") {
            Some(tag) => tag,
            None => bail!("Missing image tag"),
        };

        let mut url = self.root.join(&format!("Items/{item_id}/Images/Primary"))?;
        url.query_pairs_mut().extend_pairs([
            ("fillHeight", &height.to_string()),
            ("quality", &"96".to_string()),
            ("tag", image_tag),
        ]);

        Ok(url.to_string())
    }

    pub fn get_episode_thumbnail_or_backdrop_url(
        &self,
        item: &BaseItemDto,
        height: i32,
    ) -> Result<String> {
        let item_id = match item.id {
            Some(item_id) => item_id,
            None => bail!("Missing item ID"),
        };

        let images_tags = match item.image_tags.as_ref() {
            Some(tags) => tags,
            None => bail!("Missing images tags"),
        };

        let (url_thumb_or_backdrop, image_tag) = match images_tags.get("Thumb") {
            Some(image_tag) => ("Thumb", image_tag),
            None => (
                "Backdrop",
                match item.backdrop_image_tags.as_ref().map(|b| b.first()) {
                    Some(Some(image_tag)) => image_tag,
                    _ => bail!("Missing image tag"),
                },
            ),
        };

        let mut url = self
            .root
            .join(&format!("Items/{item_id}/Images/{url_thumb_or_backdrop}"))?;

        url.query_pairs_mut().extend_pairs([
            ("fillHeight", &height.to_string()),
            ("quality", &"96".to_string()),
            ("tag", image_tag),
        ]);

        Ok(url.to_string())
    }

    pub fn get_parent_or_item_thumbnail_url(
        &self,
        item: &BaseItemDto,
        height: i32,
    ) -> Result<String> {
        let item_id = match item
            .parent_thumb_item_id
            .or(item.parent_backdrop_item_id.or(item.id))
        {
            Some(item_id) => item_id,
            None => bail!("Missing parent backdrop item ID"),
        };

        let (url_thumb_or_backdrop, image_tag) = match item.parent_thumb_image_tag.as_ref() {
            Some(image_tag) => ("Thumb", image_tag),
            None => (
                "Backdrop",
                match item.parent_backdrop_image_tags.as_ref().map(|v| v.first()) {
                    Some(Some(image_tag)) => image_tag,
                    _ => {
                        if let Some(Some(primary_tag)) =
                            item.image_tags.as_ref().map(|i| i.get("Primary"))
                        {
                            primary_tag
                        } else {
                            bail!("Missing parent thumbnail or backdrop tag")
                        }
                    }
                },
            ),
        };

        let mut url = self
            .root
            .join(&format!("Items/{item_id}/Images/{url_thumb_or_backdrop}"))?;

        url.query_pairs_mut().extend_pairs([
            ("fillHeight", &height.to_string()),
            ("quality", &"96".to_string()),
            ("tag", image_tag),
        ]);

        Ok(url.to_string())
    }

    pub fn get_parent_or_item_primary_image_url(&self, item: &BaseItemDto) -> Result<String> {
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
            .append_pair("fillWidth", "700")
            .append_pair("quality", "100");

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
