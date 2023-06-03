use serde::Deserialize;

use super::url::httpify;

#[derive(Debug, Deserialize)]
pub struct PublicServerInfo {
    #[serde(rename = "ServerName")]
    pub server_name: String,
}

pub async fn get_public_server_info(url: &str) -> Result<PublicServerInfo, reqwest::Error> {
    let url = httpify(url);
    let url = format!("{}/System/Info/Public", url);
    let res = reqwest::get(url).await?.json().await?;
    Ok(res)
}
