use anyhow::Result;
use derive_builder::Builder;
use jellyfin_api::types::{BaseItemDto, BaseItemDtoQueryResult};
use uuid::Uuid;

use crate::jellyfin_api::api_client::ApiClient;

impl ApiClient {
    pub async fn get_latest_media(
        &self,
        parent_id: &Uuid,
        limit: Option<usize>,
    ) -> Result<Vec<BaseItemDto>> {
        let limit = limit.unwrap_or(16);

        let mut url = self
            .root
            .join(&format!("Users/{}/Items/Latest", self.account.id))?;

        url.query_pairs_mut()
            .append_pair("parentId", &parent_id.to_string())
            .append_pair("limit", &limit.to_string());

        let res = self.client.get(url).send().await?.json().await?;

        Ok(res)
    }
}

#[derive(Builder)]
pub struct GetNextUpOptions {
    #[builder(default = "16")]
    limit: usize,
    #[builder(setter(into, strip_option))]
    #[builder(default = "None")]
    series_id: Option<Uuid>,
}

impl Default for GetNextUpOptions {
    fn default() -> Self {
        Self {
            limit: 16,
            series_id: None,
        }
    }
}

impl ApiClient {
    // TODO: this can probably be combined with get_latest_media
    pub async fn get_continue_watching(
        &self,
        options: GetNextUpOptions,
    ) -> Result<Vec<BaseItemDto>> {
        let mut url = self
            .root
            .join(&format!("Users/{}/Items/Resume", self.account.id))?;

        url.query_pairs_mut()
            .append_pair("Limit", &options.limit.to_string());

        if let Some(series_id) = &options.series_id {
            url.query_pairs_mut()
                .append_pair("ParentId", &series_id.to_string());
        }

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        res.items.ok_or(anyhow::anyhow!("No items returned"))
    }

    // TODO: this can probably be combined with get_latest_media
    pub async fn get_next_up(&self, options: GetNextUpOptions) -> Result<Vec<BaseItemDto>> {
        let mut url = self.root.join("Shows/NextUp")?;

        url.query_pairs_mut()
            .append_pair("UserId", &self.account.id.to_string())
            .append_pair("Limit", &options.limit.to_string());

        if let Some(series_id) = &options.series_id {
            url.query_pairs_mut()
                .append_pair("SeriesId", &series_id.to_string());
        }

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        res.items.ok_or(anyhow::anyhow!("No items returned"))
    }
}
