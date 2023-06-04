use anyhow::Result;
use serde::Deserialize;

use super::api_client::ApiClient;

pub type UserViews = Vec<UserViewItem>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetUserViewsRes {
    pub items: UserViews,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserViewItem {
    pub id: String,
    pub name: String,
    pub collection_type: String,
}

impl ApiClient {
    pub async fn get_user_views(&self) -> Result<UserViews> {
        let url = self
            .root
            .join(&format!("Users/{}/Views", self.account.id))
            .unwrap();
        let res: GetUserViewsRes = self.client.get(url).send().await?.json().await?;
        Ok(res.items)
    }
}
