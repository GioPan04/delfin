use anyhow::Result;
use jellyfin_api::types::BaseItemDto;
use reqwest::Url;
use uuid::Uuid;

use crate::{
    config,
    jellyfin_api::{api_client::ApiClient, util::url::httpify},
};

pub fn get_stream_url(server: &config::Server, item_id: &Uuid) -> String {
    Url::parse(&httpify(&server.url))
        .unwrap()
        .join(&format!("Videos/{}/stream?static=true", item_id))
        .unwrap()
        .to_string()
}

impl ApiClient {
    pub async fn get_item(&self, item_id: &Uuid) -> Result<BaseItemDto> {
        let url = self
            .root
            .join(&format!("Users/{}/Items/{item_id}", self.account.id))?;

        let res = self.client.get(url).send().await?.json().await?;
        Ok(res)
    }
}
