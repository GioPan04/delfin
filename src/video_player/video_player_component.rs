use std::cell::OnceCell;
use std::sync::{Arc, RwLock};

use gst::ClockTime;
use gstplay::PlayState;
use gtk::prelude::*;
use relm4::{gtk, ComponentParts};
use relm4::{prelude::*, JoinHandle};

use crate::config::{Config, Server};
use crate::jellyfin_api::api::item::ItemType;
use crate::jellyfin_api::api::shows::GetEpisodesOptionsBuilder;
use crate::jellyfin_api::{api::item::get_stream_url, api_client::ApiClient, models::media::Media};
use crate::utils::ticks::ticks_to_seconds;
use crate::video_player::controls::video_player_controls::{
    VideoPlayerControls, VideoPlayerControlsInit,
};
use crate::video_player::gst_play_widget::GstVideoPlayer;

use super::controls::play_pause::{PlayPauseInput, PLAY_PAUSE_BROKER};
use super::controls::scrubber::{ScrubberInput, SCRUBBER_BROKER};
use super::controls::video_player_controls::VideoPlayerControlsInput;
use super::session::start_session_reporting;

pub struct VideoPlayer {
    config: Arc<RwLock<Config>>,
    controls: OnceCell<Controller<VideoPlayerControls>>,
    media: Option<Media>,
    api_client: Option<Arc<ApiClient>>,
    show_controls: bool,
    session_reporting_handle: Option<JoinHandle<()>>,
    player_state: VideoPlayerState,
}

#[derive(Debug, Clone, Copy)]
enum VideoPlayerState {
    Loading,
    Buffering,
    Playing { paused: bool },
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    Toast(String),
    PlayVideo(Arc<ApiClient>, Server, Box<Media>),
    ToggleControls,
    ExitPlayer,
    PlayerStateChanged(PlayState),
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[derive(Debug)]
pub enum VideoPlayerCommandOutput {
    LoadedNextPrev((Option<Media>, Option<Media>)),
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = Arc<RwLock<Config>>;
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = VideoPlayerCommandOutput;

    view! {
        #[name = "toaster"]
        adw::ToastOverlay {
            add_css_class: "video-player",

            #[name = "overlay"]
            gtk::Overlay {
                #[local_ref]
                video_player -> GstVideoPlayer {
                    add_controller = gtk::GestureClick {
                        connect_pressed[sender] => move |_, _, _, _| {
                            sender.input(VideoPlayerInput::ToggleControls);
                        },
                    },
                },

                add_overlay = &adw::HeaderBar {
                    #[watch]
                    set_visible: model.show_controls,
                    set_valign: gtk::Align::Start,
                    add_css_class: "osd",
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        #[watch]
                        set_title: if let Some(media) = &model.media {
                            &media.name
                        } else {
                            "Jellything"
                        },
                    },
                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous",
                        connect_clicked[sender] => move |_| {
                            sender.input(VideoPlayerInput::ExitPlayer);
                        },
                    },
                },

                #[name = "spinner"]
                add_overlay = &gtk::Spinner {
                    #[watch]
                    set_visible: matches!(model.player_state, VideoPlayerState::Loading | VideoPlayerState::Buffering),
                    set_spinning: true,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_width_request: 48,
                    set_height_request: 48,
                },
            },
        }

    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let config = init;

        let show_controls = true;

        let controls = OnceCell::new();

        let model = VideoPlayer {
            config,
            media: None,
            controls,
            api_client: None,
            show_controls,
            session_reporting_handle: None,
            player_state: VideoPlayerState::Loading,
        };

        let video_player = GstVideoPlayer::new();

        video_player.connect_state_changed({
            let sender = sender.clone();
            move |play_state| {
                sender.input(VideoPlayerInput::PlayerStateChanged(*play_state));
            }
        });

        video_player.connect_end_of_stream({
            let sender = sender.clone();
            move || {
                sender.input(VideoPlayerInput::ExitPlayer);
            }
        });

        video_player.connect_error({
            let sender = sender.clone();
            move |err, details| {
                println!("Video player error: {err:#?}");
                println!("Details: {details:#?}");
                sender.input(VideoPlayerInput::Toast(format!(
                    "Video player error: {}",
                    err.message()
                )));
            }
        });

        let widgets = view_output!();
        let overlay = &widgets.overlay;

        let controls = VideoPlayerControls::builder()
            .launch(VideoPlayerControlsInit {
                player: OnceCell::from(video_player),
                default_show_controls: show_controls,
            })
            .detach();
        overlay.add_overlay(controls.widget());

        model
            .controls
            .set(controls)
            .unwrap_or_else(|_| panic!("Failed to set controls"));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            VideoPlayerInput::Toast(message) => {
                let toast = adw::Toast::new(&message);
                widgets.toaster.add_toast(toast);
            }
            VideoPlayerInput::PlayVideo(api_client, server, media) => {
                let video_player = &widgets.video_player;

                self.set_player_state(VideoPlayerState::Loading);

                self.media = Some(*media.clone());
                let url = get_stream_url(&server, &media.id);
                video_player.play_uri(&url);

                let playback_position = ticks_to_seconds(media.user_data.playback_position_ticks);
                video_player.seek(ClockTime::from_seconds(playback_position as u64));

                if let Some(controls) = self.controls.get() {
                    controls.emit(VideoPlayerControlsInput::SetPlaying(Box::new(
                        *media.clone(),
                    )));
                }

                // Report start of playback
                relm4::spawn({
                    let api_client = api_client.clone();
                    let item_id = media.id.clone();
                    async move {
                        api_client.report_playback_started(&item_id).await.unwrap();
                    }
                });

                // Starts a background task that continuously reports playback progress
                self.session_reporting_handle = Some(start_session_reporting(
                    self.config.clone(),
                    api_client.clone(),
                    &media.id,
                    video_player,
                ));

                self.api_client = Some(api_client);

                self.fetch_next_prev(&sender, &media);
            }
            VideoPlayerInput::ToggleControls => {
                self.show_controls = !self.show_controls;
                let controls = self.controls.get().unwrap();
                controls.emit(VideoPlayerControlsInput::SetShowControls(
                    self.show_controls,
                ));
            }
            VideoPlayerInput::ExitPlayer => {
                widgets.video_player.stop();
                let position = widgets.video_player.position();

                // Report end of playback
                if let (Some(api_client), Some(media), Some(position)) =
                    (&self.api_client, &self.media, position)
                {
                    // Report end of playback
                    relm4::spawn({
                        let api_client = api_client.clone();
                        let item_id = media.id.clone();
                        async move {
                            api_client
                                .report_playback_stopped(&item_id, position.seconds() as usize)
                                .await
                                .unwrap();
                        }
                    });

                    self.api_client = None;
                    self.media = None;
                }

                // Stop background playback progress reporter
                if let Some(session_reporting_handle) = &self.session_reporting_handle {
                    session_reporting_handle.abort();
                    self.session_reporting_handle = None;
                }

                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
            VideoPlayerInput::PlayerStateChanged(play_state) => {
                match (&self.player_state, play_state) {
                    (_, PlayState::Playing) => {
                        self.set_player_state(VideoPlayerState::Playing { paused: false });
                    }
                    (_, PlayState::Paused) => {
                        self.set_player_state(VideoPlayerState::Playing { paused: true });
                    }
                    (VideoPlayerState::Playing { paused: _ }, PlayState::Buffering) => {
                        self.set_player_state(VideoPlayerState::Buffering);
                    }
                    _ => {}
                }
            }
        }

        self.update_view(widgets, sender);
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            VideoPlayerCommandOutput::LoadedNextPrev((prev, next)) => {
                if let Some(controls) = self.controls.get() {
                    controls.emit(VideoPlayerControlsInput::SetNextPreviousEpisodes(
                        Box::new(prev),
                        Box::new(next),
                    ));
                }
            }
        }
    }
}

impl VideoPlayer {
    fn set_player_state(&mut self, state: VideoPlayerState) {
        self.player_state = state;

        match state {
            VideoPlayerState::Loading => {
                SCRUBBER_BROKER.send(ScrubberInput::Reset);
            }
            VideoPlayerState::Playing { paused } => {
                PLAY_PAUSE_BROKER.send(PlayPauseInput::SetPlaying(!paused));
            }
            _ => {}
        }
    }

    fn fetch_next_prev(&self, sender: &ComponentSender<Self>, media: &Media) {
        if let (Some(api_client), ItemType::Episode, Some(series_id)) =
            (&self.api_client, &media.item_type, &media.series_id)
        {
            sender.oneshot_command({
                let api_client = api_client.clone();
                let series_id = series_id.clone();
                let episode_id = media.id.clone();
                async move {
                    let res = match api_client
                        .get_episodes(
                            &GetEpisodesOptionsBuilder::default()
                                .series_id(series_id)
                                .adjacent_to(&episode_id)
                                .is_missing(false)
                                .is_virtual_unaired(false)
                                .build()
                                .unwrap(),
                        )
                        .await
                    {
                        Ok(res) => res,
                        _ => {
                            return VideoPlayerCommandOutput::LoadedNextPrev((None, None));
                        }
                    };

                    match &res[..] {
                        [prev, _, next] => VideoPlayerCommandOutput::LoadedNextPrev((
                            Some(prev.clone()),
                            Some(next.clone()),
                        )),
                        [prev, cur] if cur.id == episode_id => {
                            VideoPlayerCommandOutput::LoadedNextPrev((Some(prev.clone()), None))
                        }
                        [cur, next] if cur.id == episode_id => {
                            VideoPlayerCommandOutput::LoadedNextPrev((None, Some(next.clone())))
                        }
                        _ => VideoPlayerCommandOutput::LoadedNextPrev((None, None)),
                    }
                }
            });
        }

        sender.oneshot_command(async { VideoPlayerCommandOutput::LoadedNextPrev((None, None)) });
    }
}
