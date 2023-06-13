use serde::Deserialize;

use crate::jellyfin_api::{unauthed_client::get_unauthed_client, util::url::httpify};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicServerInfo {
    pub id: String,
    pub server_name: String,
}

pub async fn get_public_server_info(url: &str) -> Result<PublicServerInfo, reqwest::Error> {
    let client = get_unauthed_client();

    let url = httpify(url);
    let url = format!("{}System/Info/Public", url);

    let res = client.get(url).send().await?.json().await?;
    Ok(res)
}
