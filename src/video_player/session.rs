use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use gst::prelude::ObjectExt;
use relm4::{tokio, JoinHandle};

use crate::{jellyfin_api::api_client::ApiClient, config::Config};

use super::gst_play_widget::GstVideoPlayer;

pub fn start_session_reporting(
    config: Arc<RwLock<Config>>,
    api_client: Arc<ApiClient>,
    item_id: &str,
    video_player: &GstVideoPlayer,
) -> JoinHandle<()> {
    let player = video_player.player().get().unwrap().downgrade();
    let item_id = item_id.to_string();

    let position_update_frequency = config
        .read()
        .unwrap()
        .video_player
        .position_update_frequency as u64;

    tokio::spawn({
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(position_update_frequency)).await;
                let player = player.upgrade().unwrap();
                let position = player.position().unwrap().seconds() as usize;
                let res = api_client.report_playback_progress("timeupdate", &item_id, position);
                res.await.unwrap();
            }
        }
    })
}
