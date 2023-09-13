use anyhow::Result;
use serde::Deserialize;

use crate::jellyfin_api::api_client::ApiClient;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Season {
    pub name: String,
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
