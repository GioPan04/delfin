use std::fmt::Display;

use anyhow::Result;
use serde::Deserialize;

use crate::jellyfin_api::{api_client::ApiClient, models::media::Media};

impl ApiClient {
    pub async fn get_latest_media(
        &self,
        parent_id: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Media>> {
        let limit = limit.unwrap_or(16);

        let mut url = self
            .root
            .join(&format!("Users/{}/Items/Latest", self.account.id))?;

        url.query_pairs_mut()
            .append_pair("parentId", parent_id)
            .append_pair("limit", &limit.to_string());

        let mut res: Vec<Media> = self.client.get(url).send().await?.json().await?;

        self.media_image_tags_to_urls(&mut res, ImageTagsType::Primary)?;

        Ok(res)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetContinueWatchingRes {
    items: Vec<Media>,
}

impl ApiClient {
    // TODO: this can probably be combined with get_latest_media
    pub async fn get_continue_watching(&self, limit: Option<usize>) -> Result<Vec<Media>> {
        let limit = limit.unwrap_or(16);

        let mut url = self
            .root
            .join(&format!("Users/{}/Items/Resume", self.account.id))?;

        url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());

        let res: GetContinueWatchingRes = self.client.get(url).send().await?.json().await?;

        let mut items = res.items;

        self.media_image_tags_to_urls(&mut items, ImageTagsType::Backdrop)?;

        Ok(items)
    }
}

enum ImageTagsType {
    Primary,
    Backdrop,
}

impl Display for ImageTagsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary => write!(f, "Primary"),
            Self::Backdrop => write!(f, "Backdrop"),
        }
    }
}

impl ApiClient {
    // Convert image tags into image URLs
    // TODO: pretty sure these are optional
    // TODO: backdrop image broken for tv shows
    fn media_image_tags_to_urls(
        &self,
        media: &mut [Media],
        image_tags_type: ImageTagsType,
    ) -> Result<()> {
        for media in media.iter_mut() {
            let mut url = self
                .root
                .join(&format!("Items/{}/Images/{image_tags_type}", media.id))?;
            url.query_pairs_mut()
                .append_pair("tag", &media.image_tags.primary)
                .append_pair("fillWidth", &200.to_string())
                .append_pair("quality", &96.to_string());
            media.image_tags.primary = url.to_string();
        }

        Ok(())
    }
}
