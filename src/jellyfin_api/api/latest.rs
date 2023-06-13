use anyhow::Result;

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

        // Parse image tags into URLs
        // TODO: pretty sure these are optional
        for media in res.iter_mut() {
            let mut url = self
                .root
                .join(&format!("Items/{}/Images/Primary", media.id))?;
            url.query_pairs_mut()
                .append_pair("tag", &media.image_tags.primary)
                // .append_pair("fillHeight", &347.to_string())
                .append_pair("fillWidth", &200.to_string())
                .append_pair("quality", &96.to_string());
            media.image_tags.primary = url.to_string();
        }

        Ok(res)
    }
}
