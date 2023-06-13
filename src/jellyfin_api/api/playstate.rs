use anyhow::Result;
use serde::Serialize;

use crate::{jellyfin_api::api_client::ApiClient, utils::ticks::seconds_to_ticks};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackStartedReq {
    pub item_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackProgressReq {
    pub event_name: String,
    pub item_id: String,
    pub position_ticks: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackStoppedReq {
    pub item_id: String,
    pub position_ticks: usize,
}

impl ApiClient {
    pub async fn report_playback_started(&self, item_id: &str) -> Result<()> {
        let url = self.root.join("Sessions/Playing").unwrap();

        self.client
            .post(url)
            .json(&ReportPlaybackStartedReq {
                item_id: item_id.into(),
            })
            .send()
            .await?;

        Ok(())
    }

    pub async fn report_playback_progress(
        &self,
        event_name: &str,
        item_id: &str,
        position_secs: usize,
    ) -> Result<()> {
        let url = self.root.join("Sessions/Playing/Progress").unwrap();

        self.client
            .post(url)
            .json(&ReportPlaybackProgressReq {
                event_name: event_name.into(),
                item_id: item_id.into(),
                position_ticks: seconds_to_ticks(position_secs),
            })
            .send()
            .await?;

        Ok(())
    }

    pub async fn report_playback_stopped(&self, item_id: &str, position_secs: usize) -> Result<()> {
        let url = self.root.join("Sessions/Playing/Stopped").unwrap();

        self.client
            .post(url)
            .json(&ReportPlaybackStoppedReq {
                item_id: item_id.into(),
                position_ticks: seconds_to_ticks(position_secs),
            })
            .send()
            .await?;

        Ok(())
    }
}
