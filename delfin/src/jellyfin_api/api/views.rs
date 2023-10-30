use anyhow::{bail, Ok, Result};
use jellyfin_api::types::{BaseItemDto, BaseItemDtoQueryResult};
use uuid::Uuid;

use crate::jellyfin_api::api_client::ApiClient;

#[derive(Clone, Debug)]
pub struct UserView {
    pub id: Uuid,
    pub name: String,
    pub collection_type: String,
}

impl TryFrom<BaseItemDto> for UserView {
    type Error = anyhow::Error;

    fn try_from(value: BaseItemDto) -> std::result::Result<Self, Self::Error> {
        if let (Some(id), Some(name), Some(collection_type)) =
            (value.id, value.name.clone(), value.collection_type.clone())
        {
            return Ok(Self {
                id,
                name,
                collection_type,
            });
        }

        bail!("UserView was missing required properties: {value:#?}");
    }
}

impl ApiClient {
    pub async fn get_user_views(&self) -> Result<Vec<UserView>> {
        let url = self
            .root
            .join(&format!("Users/{}/Views", self.account.id))
            .unwrap();
        let res: BaseItemDtoQueryResult = self.client.get(url).send().await?.json().await?;

        let items = res.items.ok_or(anyhow::anyhow!("No items returned"))?;
        items
            .iter()
            .map(|item| UserView::try_from(item.clone()))
            .collect()
    }
}
