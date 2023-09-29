use anyhow::Result;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::{globals::CONFIG, jellyfin_api::api_client::ApiClient};

impl ApiClient {
    pub fn get_stream_url(&self, item_id: &Uuid) -> String {
        if CONFIG.read().video_player.hls_playback {
            self.root
                .join(&format!(
                    "Videos/{}/main.m3u8?static=true&api_key={}",
                    item_id, self.account.access_token
                ))
                .unwrap()
                .to_string()
        } else {
            self.root
                .join(&format!("Videos/{}/stream?static=true", item_id))
                .unwrap()
                .to_string()
        }
    }

    pub async fn get_item(&self, item_id: &Uuid) -> Result<BaseItemDto> {
        let url = self
            .root
            .join(&format!("Users/{}/Items/{item_id}", self.account.id))?;

        let res = self.client.get(url).send().await?.json().await?;
        Ok(res)
    }
}
