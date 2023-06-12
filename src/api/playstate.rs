use anyhow::Result;
use serde::Serialize;

use crate::utils::ticks::seconds_to_ticks;

use super::api_client::ApiClient;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportPlaybackProgressReq {
    pub event_name: String,
    pub item_id: String,
    pub position_ticks: usize,
}

impl ApiClient {
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
}
