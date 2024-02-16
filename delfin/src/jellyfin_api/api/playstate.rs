use anyhow::Result;
use jellyfin_api::types::UserItemDataDto;
use serde::Serialize;
use uuid::Uuid;

use crate::{jellyfin_api::api_client::ApiClient, utils::ticks::seconds_to_ticks};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackStartedReq {
    pub item_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackProgressReq {
    pub event_name: String,
    pub item_id: Uuid,
    pub position_ticks: usize,
    pub is_paused: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackStoppedReq {
    pub item_id: Uuid,
    pub position_ticks: usize,
}

impl ApiClient {
    pub async fn report_playback_started(&self, item_id: Uuid) -> Result<()> {
        let url = self.root.join("Sessions/Playing").unwrap();

        self.client
            .post(url)
            .json(&ReportPlaybackStartedReq { item_id })
            .send()
            .await?;

        Ok(())
    }

    pub async fn report_playback_progress(
        &self,
        event_name: &str,
        item_id: Uuid,
        position_secs: usize,
        is_paused: bool,
    ) -> Result<()> {
        let url = self.root.join("Sessions/Playing/Progress").unwrap();

        self.client
            .post(url)
            .json(&ReportPlaybackProgressReq {
                event_name: event_name.into(),
                item_id,
                position_ticks: seconds_to_ticks(position_secs),
                is_paused,
            })
            .send()
            .await?;

        Ok(())
    }

    pub async fn report_playback_stopped(&self, item_id: Uuid, position_secs: usize) -> Result<()> {
        let url = self.root.join("Sessions/Playing/Stopped").unwrap();

        self.client
            .post(url)
            .json(&ReportPlaybackStoppedReq {
                item_id,
                position_ticks: seconds_to_ticks(position_secs),
            })
            .send()
            .await?;

        Ok(())
    }

    pub async fn mark_item_played(&self, item_id: Uuid) -> Result<UserItemDataDto> {
        let url = self
            .root
            .join(&format!("Users/{}/PlayedItems/{item_id}", self.account.id))?;
        Ok(self.client.post(url).send().await?.json().await?)
    }

    pub async fn mark_item_unplayed(&self, item_id: Uuid) -> Result<UserItemDataDto> {
        let url = self
            .root
            .join(&format!("Users/{}/PlayedItems/{item_id}", self.account.id))?;
        Ok(self.client.delete(url).send().await?.json().await?)
    }
}
