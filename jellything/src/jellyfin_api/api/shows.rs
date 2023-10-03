use anyhow::Result;
use derive_builder::Builder;
use jellyfin_api::types::{BaseItemDto, BaseItemDtoQueryResult};
use uuid::Uuid;

use crate::jellyfin_api::api_client::ApiClient;

impl ApiClient {
    pub async fn get_seasons(&self, series_id: &Uuid) -> Result<Vec<BaseItemDto>> {
        let mut url = self.root.join(&format!("Shows/{series_id}/Seasons"))?;

        url.query_pairs_mut()
            .append_pair("userId", &self.account.id);

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        res.items.ok_or(anyhow::anyhow!("No items returned"))
    }
}

#[derive(Builder)]
pub struct GetEpisodesOptions {
    series_id: Uuid,
    #[builder(default = "None")]
    #[builder(setter(into, strip_option))]
    season_id: Option<Uuid>,
    #[builder(default = "None")]
    #[builder(setter(into, strip_option))]
    is_virtual_unaired: Option<bool>,
    #[builder(default = "None")]
    #[builder(setter(into, strip_option))]
    is_missing: Option<bool>,
    #[builder(setter(into, strip_option))]
    #[builder(default = "None")]
    adjacent_to: Option<Uuid>,
}

impl ApiClient {
    pub async fn get_episodes(&self, options: &GetEpisodesOptions) -> Result<Vec<BaseItemDto>> {
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
            url.query_pairs_mut()
                .append_pair("seasonId", &season_id.to_string());
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

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        res.items.ok_or(anyhow::anyhow!("No items returned"))
    }
}
