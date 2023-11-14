use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use crate::{
    globals::CONFIG, jellyfin_api::api_client::ApiClient, library::LIBRARY_REFRESH_QUEUED,
    media_details::MEDIA_DETAILS_REFRESH_QUEUED,
};

use super::backends::VideoPlayerBackend;

#[derive(Default)]
pub struct SessionPlaybackReporter(Option<Uuid>);

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
        if let Some(id) = self.0.take() {
            video_player.borrow_mut().disconnect_signal_handler(&id);
        }
    }
}

fn start_session_reporting(
    api_client: Arc<ApiClient>,
    item_id: &Uuid,
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
) -> Uuid {
    let config = CONFIG.read();

    let position_update_frequency = config.video_player.position_update_frequency;

    *LIBRARY_REFRESH_QUEUED.write() = true;
    *MEDIA_DETAILS_REFRESH_QUEUED.write() = true;

    video_player
        .borrow_mut()
        .connect_position_updated(Box::new({
            let item_id = *item_id;
            let last_update = RwLock::<usize>::new(0);

            move |position| {
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
                        let mut last_update =
                            last_update.write().expect("Error writing last_update");
                        *last_update = position;

                        tokio::spawn({
                            let api_client = api_client.clone();
                            async move {
                                if (api_client
                                    .report_playback_progress("timeupdate", &item_id, position)
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
        }))
}
