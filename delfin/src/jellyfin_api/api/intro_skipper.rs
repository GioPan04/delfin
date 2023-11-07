use std::ops::Range;

use anyhow::Result;
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::jellyfin_api::api_client::ApiClient;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IntroTimestamps {
    pub intro_start: f32,
    pub intro_end: f32,
    pub show_skip_prompt_at: f32,
    pub hide_skip_prompt_at: f32,
}

impl IntroTimestamps {
    pub fn range_show(&self) -> Range<f32> {
        Range {
            start: self.show_skip_prompt_at,
            end: self.hide_skip_prompt_at,
        }
    }
}

impl ApiClient {
    pub async fn get_intro_timestamps(&self, episode_id: &Uuid) -> Result<Option<IntroTimestamps>> {
        let url = self
            .root
            .join(&format!("Episode/{episode_id}/IntroTimestamps"))?;

        let res = self.client.get(url).send().await?;
        if res.status() == StatusCode::NOT_FOUND {
            // Intro skipper returns a 404 if episode doesn't have timestamps
            return Ok(None);
        }

        Ok(Some(res.json().await?))
    }
}
