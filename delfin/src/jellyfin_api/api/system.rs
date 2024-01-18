use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::jellyfin_api::{
    api_client::ApiClient, unauthed_client::get_unauthed_client, util::url::httpify,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    pub id: Uuid,
    pub server_name: String,
}

pub async fn get_public_server_info(url: &str) -> Result<PublicServerInfo, reqwest::Error> {
    let client = get_unauthed_client();

    let url = httpify(url);
    let url = format!("{}System/Info/Public", url);

    let res = client.get(url).send().await?.json().await?;
    Ok(res)
}

impl ApiClient {
    pub async fn ping(&self) -> Result<String> {
        let url = self.root.join("System/Ping")?;
        let res = self
            .client
            .get(url)
            .send()
            .await?
            // Error unless we get a 200
            .error_for_status()?
            .text()
            .await?;
        Ok(res)
    }
}
