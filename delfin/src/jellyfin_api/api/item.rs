use anyhow::{Context, Result};
use jellyfin_api::types::{
    BaseItemDto, BaseItemDtoQueryResult, PlaybackInfoDto, PlaybackInfoResponse,
};
use uuid::Uuid;

use crate::{
    globals::CONFIG, jellyfin_api::api_client::ApiClient,
    utils::device_profile::DEVICE_PROFILE_DIRECT_PLAY,
};

impl ApiClient {
    pub fn get_stream_url(&self, item_id: &Uuid) -> String {
        if CONFIG.read().video_player.hls_playback {
            self.root
                .join(&format!(
                    "Videos/{}/main.m3u8?static=true&api_key={}",
                    item_id, self.account.access_token
                ))
                .unwrap()
                .to_string()
        } else {
            self.root
                .join(&format!("Videos/{}/stream?static=true", item_id))
                .unwrap()
                .to_string()
        }
    }

    pub async fn get_item(&self, item_id: &Uuid) -> Result<BaseItemDto> {
        let url = self
            .root
            .join(&format!("Users/{}/Items/{item_id}", self.account.id))?;

        let res = self.client.get(url).send().await?.json().await?;
        Ok(res)
    }

    pub async fn search_items(
        &self,
        search_term: &str,
        start_index: usize,
        limit: usize,
    ) -> Result<(Vec<BaseItemDto>, usize)> {
        let mut url = self
            .root
            .join(&format!("Users/{}/Items", self.account.id))?;
        url.query_pairs_mut()
            .append_pair("SearchTerm", search_term)
            .append_pair("IncludeItemTypes", "Series,Movie")
            .append_pair("SortBy", "SortName,ProductionYear")
            .append_pair("SortOrder", "Ascending")
            .append_pair("Recursive", "true")
            .append_pair("StartIndex", &start_index.to_string())
            .append_pair("Limit", &limit.to_string());

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        let items = res.items.context("No items returned")?;
        let total_record_count = res
            .total_record_count
            .context("Total record count not returned")?;

        Ok((items, total_record_count as usize))
    }

    pub async fn get_search_suggestions(&self, limit: usize) -> Result<(Vec<BaseItemDto>, usize)> {
        let mut url = self
            .root
            .join(&format!("Users/{}/Items", self.account.id))?;
        url.query_pairs_mut()
            .append_pair("IncludeItemTypes", "Series,Movie")
            .append_pair("SortBy", "Random")
            .append_pair("Recursive", "true")
            .append_pair("Limit", &limit.to_string());

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        let items = res.items.context("No items returned")?;
        let total_record_count = res
            .total_record_count
            .context("Total record count not returned")?;

        Ok((items, total_record_count as usize))
    }

    pub async fn get_playback_info(&self, item_id: &Uuid) -> Result<PlaybackInfoResponse> {
        let mut url = self
            .root
            .join(&format!("Items/{item_id}/PlaybackInfo"))
            .unwrap();
        url.query_pairs_mut()
            .append_pair("userId", &self.account.id);

        let body: PlaybackInfoDto = PlaybackInfoDto::builder()
            .device_profile(Some(DEVICE_PROFILE_DIRECT_PLAY.clone()))
            .try_into()
            .unwrap();

        Ok(self
            .client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?)
    }
}
