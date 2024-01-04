use anyhow::{bail, Context, Ok, Result};
use jellyfin_api::types::{BaseItemDto, BaseItemDtoQueryResult};
use uuid::Uuid;

use crate::jellyfin_api::{api_client::ApiClient, models::collection_type::CollectionType};

#[derive(Clone, Debug)]
pub struct UserView {
    pub id: Uuid,
    pub name: String,
    pub collection_type: CollectionType,
}

impl TryFrom<BaseItemDto> for UserView {
    type Error = anyhow::Error;

    fn try_from(value: BaseItemDto) -> std::result::Result<Self, Self::Error> {
        if let (Some(id), Some(name)) = (value.id, value.name.clone()) {
            return Ok(Self {
                id,
                name,
                collection_type: value.collection_type.into(),
            });
        }

        bail!("UserView was missing required properties: {value:#?}");
    }
}

impl ApiClient {
    pub async fn get_user_views(
        &self,
        start_index: Option<usize>,
        limit: Option<usize>,
    ) -> Result<(Vec<BaseItemDto>, usize)> {
        let mut url = self
            .root
            .join(&format!("Users/{}/Views", self.account.id))
            .unwrap();

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(start_index) = start_index {
                query_pairs.append_pair("StartIndex", &start_index.to_string());
            }
            if let Some(limit) = limit {
                query_pairs.append_pair("Limit", &limit.to_string());
            }
        }

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        let items = res.items.context("No items returned")?;
        let total_record_count = res
            .total_record_count
            .context("Total record count not returned")?;

        Ok((items, total_record_count as usize))
    }

    pub async fn get_view_items(
        &self,
        view: &UserView,
        start_index: usize,
        limit: usize,
    ) -> Result<(Vec<BaseItemDto>, usize)> {
        let mut url = self
            .root
            .join(&format!("Users/{}/Items", self.account.id))
            .unwrap();

        url.query_pairs_mut()
            .append_pair("ParentId", &view.id.to_string())
            .append_pair("SortBy", "SortName,ProductionYear")
            .append_pair("SortOrder", "Ascending")
            .append_pair("Recursive", "true")
            .append_pair("StartIndex", &start_index.to_string())
            .append_pair("Limit", &limit.to_string());

        if let Some(item_type) = view.collection_type.item_type() {
            url.query_pairs_mut()
                .append_pair("IncludeItemTypes", &item_type.to_string());
        }

        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        let items = res.items.context("No items returned")?;
        let total_record_count = res
            .total_record_count
            .context("Total record count not returned")?;

        Ok((items, total_record_count as usize))
    }
}
