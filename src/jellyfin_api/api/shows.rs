use anyhow::Result;
use derive_builder::Builder;
use serde::Deserialize;

use crate::{
    jellyfin_api::{api_client::ApiClient, models::media::Media},
    media_details::episode::EPISODE_THUMBNAIL_SIZE,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSeasonsRes {
    items: Vec<Media>,
}

impl ApiClient {
    pub async fn get_seasons(&self, series_id: &str) -> Result<Vec<Media>> {
        let mut url = self.root.join(&format!("Shows/{series_id}/Seasons"))?;

        url.query_pairs_mut()
            .append_pair("userId", &self.account.id);

        let res: GetSeasonsRes = self.client.get(url).send().await?.json().await?;

        Ok(res.items)
    }
}

#[derive(Builder)]
pub struct GetEpisodesOptions {
    series_id: String,
    #[builder(default = "None")]
    #[builder(setter(into, strip_option))]
    season_id: Option<String>,
    #[builder(default = "None")]
    #[builder(setter(into, strip_option))]
    is_virtual_unaired: Option<bool>,
    #[builder(default = "None")]
    #[builder(setter(into, strip_option))]
    is_missing: Option<bool>,
    #[builder(setter(into, strip_option))]
    #[builder(default = "None")]
    adjacent_to: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetEpisodesRes {
    items: Vec<Media>,
}

impl ApiClient {
    pub async fn get_episodes(&self, options: &GetEpisodesOptions) -> Result<Vec<Media>> {
        let GetEpisodesOptions {
            series_id,
            season_id,
            is_virtual_unaired,
            is_missing,
            adjacent_to,
        } = options;

        let mut url = self.root.join(&format!("Shows/{series_id}/Episodes"))?;

        url.query_pairs_mut()
            .append_pair("userId", &self.account.id)
            .append_pair("fields", "Overview");

        if let Some(season_id) = season_id {
            url.query_pairs_mut().append_pair("seasonId", season_id);
        }
        if let Some(is_virtual_unaired) = is_virtual_unaired {
            url.query_pairs_mut()
                .append_pair("isVirtualUnaired", &is_virtual_unaired.to_string());
        }
        if let Some(is_missing) = is_missing {
            url.query_pairs_mut()
                .append_pair("isMissing", &is_missing.to_string());
        }
        if let Some(adjacent_to) = adjacent_to {
            url.query_pairs_mut()
                .append_pair("adjacentTo", &adjacent_to.to_string());
        }

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
