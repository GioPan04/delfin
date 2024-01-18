use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use uuid::Uuid;

use crate::{
    globals::CONFIG, jellyfin_api::api_client::ApiClient, library::LIBRARY_REFRESH_QUEUED,
    media_details::MEDIA_DETAILS_REFRESH_QUEUED,
};

use super::backends::{PlayerState, VideoPlayerBackend};

#[derive(Default)]
pub struct SessionPlaybackReporter(Option<(Uuid, Uuid)>);

impl SessionPlaybackReporter {
    pub fn start(
        &mut self,
        api_client: Arc<ApiClient>,
        item_id: &Uuid,
        video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    ) {
        self.stop(video_player.clone());
        self.0 = Some(start_session_reporting(api_client, item_id, video_player));
    }

    pub fn stop(&mut self, video_player: Arc<RefCell<dyn VideoPlayerBackend>>) {
        if let Some((position_updated_signal_handler_id, player_state_signal_handler_id)) =
            self.0.take()
        {
            let mut video_player = video_player.borrow_mut();
            video_player.disconnect_signal_handler(&position_updated_signal_handler_id);
            video_player.disconnect_signal_handler(&player_state_signal_handler_id);
        }
    }
}

fn start_session_reporting(
    api_client: Arc<ApiClient>,
    item_id: &Uuid,
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
) -> (Uuid, Uuid) {
    let config = CONFIG.read();

    let position_update_frequency = config.video_player.position_update_frequency;

    *LIBRARY_REFRESH_QUEUED.write() = true;
    *MEDIA_DETAILS_REFRESH_QUEUED.write() = true;

    let position = Arc::new(AtomicUsize::default());
    let is_paused = Arc::new(AtomicBool::new(false));

    let mut video_player = video_player.borrow_mut();

    let position_updated_signal_handler_id = video_player.connect_position_updated(Box::new({
        let api_client = api_client.clone();
        let item_id = *item_id;
        let last_update = RwLock::<usize>::new(0);
        let position_store = position.clone();
        let is_paused = is_paused.clone();

        move |position| {
            position_store.store(position, Ordering::Relaxed);

            // Avoid deadlocks
            let last_update_val = match last_update.try_read() {
                Ok(val) => *val,
                Err(_) => return,
            };

            // If user rewinds, this subtraction will underflow. We pass
            // position_update_frequency here so that it'll be the default value for diff,
            // causing a position update if the subtraction underflowed.
            match (
                position.checked_sub(last_update_val),
                position_update_frequency,
            ) {
                (None, diff) | (Some(diff), _) if diff >= position_update_frequency => {
                    let mut last_update = last_update.write().expect("Error writing last_update");
                    *last_update = position;

                    tokio::spawn({
                        let api_client = api_client.clone();
                        let is_paused = is_paused.clone();
                        async move {
                            if (api_client
                                .report_playback_progress(
                                    "timeupdate",
                                    item_id,
                                    position,
                                    is_paused.load(Ordering::Relaxed),
                                )
                                .await)
                                .is_err()
                            {
                                println!("Error reporting playback progress.");
                            }
                        }
                    });
                }
                _ => {}
            }
        }
    }));

    let player_state_signal_handler_id = video_player.connect_player_state_changed(Box::new({
        let api_client = api_client.clone();
        let item_id = *item_id;
        let position = position.clone();
        let is_paused = is_paused.clone();

        move |player_state| {
            if let PlayerState::Playing { paused } = player_state {
                is_paused.store(paused, Ordering::Relaxed);

                tokio::spawn({
                    let api_client = api_client.clone();
                    let position = position.clone();
                    let is_paused = is_paused.clone();
                    async move {
                        if (api_client
                            .report_playback_progress(
                                "timeupdate",
                                item_id,
                                position.load(Ordering::Relaxed),
                                is_paused.load(Ordering::Relaxed),
                            )
                            .await)
                            .is_err()
                        {
                            println!("Error reporting playback progress.");
                        }
                    }
                });
            }
        }
    }));

    (
        position_updated_signal_handler_id,
        player_state_signal_handler_id,
    )
}
