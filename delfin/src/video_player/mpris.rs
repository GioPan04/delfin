use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    time::Duration,
};

use jellyfin_api::types::BaseItemDto;
use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig,
    SeekDirection,
};
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::api_client::ApiClient,
    utils::item_name::ItemName,
    video_player::controls::next_prev_episode::{
        NextPrevEpisodeInput, NEXT_EPISODE_BROKER, PREV_EPISODE_BROKER,
    },
};

use super::{
    backends::{PlayerState, VideoPlayerBackend},
    controls::{
        play_pause::{PlayPauseInput, PLAY_PAUSE_BROKER},
        skip_forwards_backwards::{
            SkipForwardsBackwardsInput, SKIP_BACKWARDS_BROKER, SKIP_FORWARDS_BROKER,
        },
    },
};

#[derive(Debug)]
enum MprisInput {
    Play,
    Pause,
    Duration(usize),
    Position(usize),
    Close,
}

pub struct MprisPlaybackReporter {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    signal_handler_ids: Vec<Uuid>,
    tx: UnboundedSender<MprisInput>,
}

impl MprisPlaybackReporter {
    pub fn new(
        api_client: Arc<ApiClient>,
        item: BaseItemDto,
        video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    ) -> Self {
        let config = PlatformConfig {
            dbus_name: "delfin",
            display_name: "Delfin",
            // Will need to pass a window handle for Windows support
            hwnd: None,
        };

        let mut controls = MediaControls::new(config).expect("Failed creating MediaControls");
        attach_media_control_events(&mut controls);
        let controls = Arc::new(Mutex::new(controls));

        let (tx, mut rx) = mpsc::unbounded_channel::<MprisInput>();

        let mut signal_handler_ids = Vec::new();

        signal_handler_ids.push(
            video_player
                .borrow_mut()
                .connect_player_state_changed(Box::new({
                    let tx = tx.clone();
                    move |player_state| {
                        if tx.is_closed() {
                            return;
                        }
                        if let PlayerState::Playing { paused } = player_state {
                            tx.send(if paused {
                                MprisInput::Pause
                            } else {
                                MprisInput::Play
                            })
                            .expect("Failed to update MPRIS state");
                        }
                    }
                })),
        );

        signal_handler_ids.push(
            video_player
                .borrow_mut()
                .connect_duration_updated(Box::new({
                    let tx = tx.clone();
                    move |duration| {
                        if tx.is_closed() {
                            return;
                        }
                        tx.send(MprisInput::Duration(duration))
                            .expect("Failed to update MPRIS duration");
                    }
                })),
        );

        signal_handler_ids.push(
            video_player
                .borrow_mut()
                .connect_position_updated(Box::new({
                    let tx = tx.clone();
                    move |position| {
                        if tx.is_closed() {
                            return;
                        }
                        tx.send(MprisInput::Position(position))
                            .expect("Failed to update MPRIS position");
                    }
                })),
        );

        tokio::spawn({
            let controls = controls.clone();
            async move {
                let title = item.episode_name_with_number();
                let series_name = item.series_name.clone();
                let cover_url = api_client.get_next_up_thumbnail_url(&item).ok();
                let metadata = MediaMetadata {
                    title: title.as_deref(),
                    album: series_name.as_deref(),
                    cover_url: cover_url.as_deref(),
                    ..Default::default()
                };

                controls
                    .lock()
                    .unwrap()
                    .set_metadata(metadata.clone())
                    .expect("Failed to set MPRIS metadata");

                let mut paused = false;

                while let Some(msg) = rx.recv().await {
                    let mut controls = controls
                        .lock()
                        .expect("Failed to acquire lock on MPRIS media controls.");

                    match msg {
                        MprisInput::Play => {
                            controls
                                .set_playback(MediaPlayback::Playing { progress: None })
                                .expect("Error setting MPRIS playback");
                            paused = false;
                        }
                        MprisInput::Pause => {
                            controls
                                .set_playback(MediaPlayback::Paused { progress: None })
                                .expect("Error setting MPRIS playback");
                            paused = true;
                        }
                        MprisInput::Duration(duration) => {
                            controls
                                .set_metadata(MediaMetadata {
                                    duration: Some(Duration::from_secs(duration as u64)),
                                    ..metadata.clone()
                                })
                                .expect("Error setting MPRIS metadata");
                        }
                        MprisInput::Position(position) => {
                            controls
                                .set_playback(if paused {
                                    MediaPlayback::Paused {
                                        progress: Some(MediaPosition(Duration::from_secs(
                                            position as u64,
                                        ))),
                                    }
                                } else {
                                    MediaPlayback::Playing {
                                        progress: Some(MediaPosition(Duration::from_secs(
                                            position as u64,
                                        ))),
                                    }
                                })
                                .expect("Error setting MPRIS playback");
                        }
                        MprisInput::Close => {
                            rx.close();
                            return;
                        }
                    }
                }
            }
        });

        Self {
            video_player,
            signal_handler_ids: Vec::default(),
            tx,
        }
    }
}

impl Drop for MprisPlaybackReporter {
    fn drop(&mut self) {
        for id in self.signal_handler_ids.drain(0..) {
            self.video_player
                .borrow_mut()
                .disconnect_signal_handler(&id);
        }

        self.tx
            .send(MprisInput::Close)
            .expect("Failed to close MPRIS playback reporter channel");
    }
}

fn attach_media_control_events(controls: &mut MediaControls) {
    controls
        .attach(|event: MediaControlEvent| {
            match event {
                MediaControlEvent::Play => {
                    PLAY_PAUSE_BROKER.send(PlayPauseInput::SetPlaying(true));
                }
                MediaControlEvent::Pause => {
                    PLAY_PAUSE_BROKER.send(PlayPauseInput::SetPlaying(false));
                }
                MediaControlEvent::Toggle => {
                    PLAY_PAUSE_BROKER.send(PlayPauseInput::TogglePlaying);
                }
                MediaControlEvent::SeekBy(direction, amount) => match direction {
                    SeekDirection::Forward => {
                        SKIP_FORWARDS_BROKER.send(SkipForwardsBackwardsInput::SkipByAmount(amount));
                    }
                    SeekDirection::Backward => {
                        SKIP_BACKWARDS_BROKER
                            .send(SkipForwardsBackwardsInput::SkipByAmount(amount));
                    }
                },
                MediaControlEvent::Seek(direction) => match direction {
                    SeekDirection::Forward => {
                        SKIP_FORWARDS_BROKER.send(SkipForwardsBackwardsInput::Skip);
                    }
                    SeekDirection::Backward => {
                        SKIP_BACKWARDS_BROKER.send(SkipForwardsBackwardsInput::Skip);
                    }
                },
                MediaControlEvent::SetPosition(MediaPosition(position)) => {
                    SKIP_BACKWARDS_BROKER.send(SkipForwardsBackwardsInput::SkipTo(position));
                }
                MediaControlEvent::Previous => {
                    PREV_EPISODE_BROKER.send(NextPrevEpisodeInput::Play);
                }
                MediaControlEvent::Next => {
                    NEXT_EPISODE_BROKER.send(NextPrevEpisodeInput::Play);
                }
                MediaControlEvent::Stop | MediaControlEvent::Quit => {
                    APP_BROKER.send(AppInput::NavigateBack);
                }
                MediaControlEvent::Raise => {
                    APP_BROKER.send(AppInput::Present);
                }
                MediaControlEvent::OpenUri(_) => {
                    // not supported
                }
            }
        })
        .unwrap();
}
