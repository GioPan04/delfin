use anyhow::Result;
use serde::Deserialize;

use crate::{
    jellyfin_api::{
        api_client::ApiClient,
        models::media::{Media, UserData},
    },
    media_details::episode::EPISODE_THUMBNAIL_SIZE,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Season {
    pub id: String,
    pub name: String,
    pub user_data: UserData,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSeasonsRes {
    items: Vec<Season>,
}

impl ApiClient {
    pub async fn get_seasons(&self, series_id: &str) -> Result<Vec<Season>> {
        let mut url = self.root.join(&format!("Shows/{series_id}/Seasons"))?;

        url.query_pairs_mut()
            .append_pair("userId", &self.account.id);

        let res: GetSeasonsRes = self.client.get(url).send().await?.json().await?;

        Ok(res.items)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetEpisodesRes {
    items: Vec<Media>,
}

impl ApiClient {
    pub async fn get_episodes(&self, series_id: &str, season_id: &str) -> Result<Vec<Media>> {
        let mut url = self.root.join(&format!("Shows/{series_id}/Episodes"))?;

        url.query_pairs_mut()
            .append_pair("userId", &self.account.id)
            .append_pair("seasonId", season_id)
            .append_pair("fields", "Overview");

        let res: GetEpisodesRes = self.client.get(url).send().await?.json().await?;
        let mut items = res.items;

        self.episode_image_tags_to_urls(&mut items)?;

        Ok(items)
    }

    fn episode_image_tags_to_urls(&self, media: &mut [Media]) -> Result<()> {
        for media in media.iter_mut() {
            let item_id = media.id.clone();

            let mut url = self.root.join(&format!("Items/{item_id}/Images/Primary"))?;

            url.query_pairs_mut()
                .append_pair("tag", &media.image_tags.primary)
                .append_pair("fillHeight", &EPISODE_THUMBNAIL_SIZE.to_string())
                .append_pair("quality", &96.to_string());
            media.image_tags.primary = url.to_string();
        }

        Ok(())
    }
}
