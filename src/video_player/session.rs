use std::sync::{Arc, RwLock};

use gst::prelude::ObjectExt;
use tokio::{
    task::JoinHandle,
    time::{sleep, Duration},
};
use uuid::Uuid;

use crate::{config::Config, jellyfin_api::api_client::ApiClient};

use super::gst_play_widget::GstVideoPlayer;

pub fn start_session_reporting(
    config: Arc<RwLock<Config>>,
    api_client: Arc<ApiClient>,
    item_id: &Uuid,
    video_player: &GstVideoPlayer,
) -> JoinHandle<()> {
    let player = video_player.player().get().unwrap().downgrade();

    let position_update_frequency = config
        .read()
        .unwrap()
        .video_player
        .position_update_frequency as u64;

    tokio::spawn({
        let item_id = *item_id;
        async move {
            loop {
                sleep(Duration::from_secs(position_update_frequency)).await;
                let player = player.upgrade().unwrap();
                let position = player.position().unwrap().seconds() as usize;
                let res = api_client.report_playback_progress("timeupdate", &item_id, position);
                res.await.unwrap();
            }
        }
    })
}
