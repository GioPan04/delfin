use std::cell::RefCell;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

use adw::prelude::*;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::{gtk, ComponentParts};
use relm4::{prelude::*, JoinHandle};

use crate::app::{AppInput, APP_BROKER};
use crate::globals::CONFIG;
use crate::jellyfin_api::api::shows::GetEpisodesOptionsBuilder;
use crate::jellyfin_api::api_client::ApiClient;
use crate::utils::ticks::ticks_to_seconds;
use crate::video_player::controls::skip_forwards_backwards::{
    SkipForwardsBackwardsInput, SKIP_BACKWARDS_BROKER, SKIP_FORWARDS_BROKER,
};
use crate::video_player::controls::video_player_controls::{
    VideoPlayerControls, VideoPlayerControlsInit,
};
use crate::video_player::next_up::{NextUp, NEXT_UP_VISIBILE};

use super::backends::{PlayerState, VideoPlayerBackend};
use super::controls::play_pause::{PlayPauseInput, PLAY_PAUSE_BROKER};
use super::controls::scrubber::{ScrubberInput, SCRUBBER_BROKER};
use super::controls::video_player_controls::VideoPlayerControlsInput;
use super::next_up::NextUpInput;
use super::session::start_session_reporting;

pub struct VideoPlayer {
    backend: Arc<RefCell<dyn VideoPlayerBackend>>,
    media: Option<BaseItemDto>,
    api_client: Option<Arc<ApiClient>>,
    hiding: Arc<AtomicBool>,

    show_controls: bool,
    show_controls_locked: bool,
    session_reporting_handle: Option<JoinHandle<()>>,
    player_state: PlayerState,
    next: Option<BaseItemDto>,

    controls: Controller<VideoPlayerControls>,
    next_up: Controller<NextUp>,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    Toast(String),
    PlayVideo(Arc<ApiClient>, Box<BaseItemDto>),
    SetShowControls { show: bool, locked: bool },
    ToggleControls,
    EndOfStream,
    StopPlayer,
    PlayerStateChanged(PlayerState),
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[derive(Debug)]
pub enum VideoPlayerCommandOutput {
    LoadedNextPrev((Option<BaseItemDto>, Option<BaseItemDto>)),
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = ();
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = VideoPlayerCommandOutput;

    view! {
        adw::NavigationPage {
            #[watch]
            set_title: &model.media.as_ref()
                .and_then(|media| media.name.clone())
                .unwrap_or("Delfin".to_string()),

            #[wrap(Some)]
            #[name = "toaster"]
            set_child = &adw::ToastOverlay {
                add_css_class: "video-player",

                gtk::Overlay {
                    #[local_ref]
                    video_player -> gtk::Widget {
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
                    },

                    #[name = "spinner"]
                    add_overlay = &gtk::Spinner {
                        #[watch]
                        set_visible: matches!(model.player_state, PlayerState::Loading | PlayerState::Buffering),
                        set_spinning: true,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        set_width_request: 48,
                        set_height_request: 48,
                    },

                    add_overlay: model.controls.widget(),

                    add_overlay: model.next_up.widget(),
                },
            },

            connect_hiding[sender] => move |_| {
                sender.input(VideoPlayerInput::StopPlayer);
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let show_controls = true;

        let backend: Arc<RefCell<dyn VideoPlayerBackend>> =
            CONFIG.read().video_player.backend.into();

        let controls = VideoPlayerControls::builder()
            .launch(VideoPlayerControlsInit {
                player: backend.clone(),
                default_show_controls: show_controls,
            })
            .detach();

        let next_up = NextUp::builder().launch(backend.clone()).detach();

        let model = VideoPlayer {
            backend,
            media: None,
            api_client: None,
            hiding: Arc::new(AtomicBool::new(false)),

            show_controls,
            show_controls_locked: false,
            session_reporting_handle: None,
            player_state: PlayerState::Loading,
            next: None,

            controls,
            next_up,
        };

        model.backend.borrow_mut().connect_player_state_changed({
            let sender = sender.clone();
            Box::new(move |state| {
                if cfg!(debug_assertions) {
                    println!("Player state changed: {state:#?}");
                }

                sender.input(VideoPlayerInput::PlayerStateChanged(state));
            })
        });

        model.backend.borrow_mut().connect_end_of_stream({
            let sender = sender.clone();
            let hiding = model.hiding.clone();
            Box::new(move || {
                if !hiding.load(atomic::Ordering::Relaxed) {
                    sender.input(VideoPlayerInput::EndOfStream);
                }
            })
        });

        // TODO
        // video_player.connect_error({
        //     let sender = sender.clone();
        //     move |err, details| {
        //         println!("Video player error: {err:#?}");
        //         println!("Details: {details:#?}");
        //         sender.input(VideoPlayerInput::Toast(format!(
        //             "Video player error: {}",
        //             err.message()
        //         )));
        //     }
        // });

        let binding = model.backend.clone();
        let binding = binding.borrow();
        let video_player = binding.widget();

        let widgets = view_output!();

        NEXT_UP_VISIBILE.subscribe(sender.input_sender(), |visible| {
            VideoPlayerInput::SetShowControls {
                show: *visible,
                locked: *visible,
            }
        });

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
            VideoPlayerInput::PlayVideo(api_client, item) => {
                self.set_player_state(PlayerState::Loading);
                self.next = None;
                self.hiding.store(false, atomic::Ordering::Relaxed);

                self.media = Some(*item.clone());
                let url = api_client.get_stream_url(&item.id.unwrap());
                self.backend.borrow_mut().play_uri(&url);

                if let Some(playback_position_ticks) = item
                    .user_data
                    .as_ref()
                    .and_then(|user_data| user_data.playback_position_ticks)
                {
                    let playback_position = ticks_to_seconds(playback_position_ticks);
                    self.backend.borrow().seek_to(playback_position as usize);
                }

                self.controls
                    .emit(VideoPlayerControlsInput::SetPlaying(Box::new(
                        *item.clone(),
                    )));

                // Report start of playback
                if let Some(item_id) = item.id {
                    relm4::spawn({
                        let api_client = api_client.clone();
                        async move {
                            api_client
                                .report_playback_started(&item_id.to_string())
                                .await
                                .unwrap();
                        }
                    });
                }

                // Starts a background task that continuously reports playback progress
                start_session_reporting(
                    api_client.clone(),
                    &item.id.unwrap(),
                    self.backend.clone(),
                );

                self.api_client = Some(api_client);

                self.fetch_next_prev(&sender, &item);
            }
            VideoPlayerInput::SetShowControls { show, locked } => {
                self.show_controls = show;
                self.show_controls_locked = locked;
                self.controls
                    .emit(VideoPlayerControlsInput::SetShowControls(!locked && show));
            }
            VideoPlayerInput::ToggleControls => {
                if self.show_controls_locked {
                    return;
                }

                sender.input(VideoPlayerInput::SetShowControls {
                    show: !self.show_controls,
                    locked: false,
                });
            }
            VideoPlayerInput::EndOfStream => {
                match self.player_state {
                    PlayerState::Playing { paused: _ } => {}
                    _ => {
                        return;
                    }
                };

                // Play next episode if available
                if let Some(next) = &self.next {
                    APP_BROKER.send(AppInput::PlayVideo(next.clone()));
                    return;
                }

                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
            VideoPlayerInput::StopPlayer => {
                self.hiding.store(true, atomic::Ordering::Relaxed);

                self.backend.borrow_mut().stop();

                let position = self.backend.borrow().position();

                // Report end of playback
                if let (Some(api_client), Some(media)) = (&self.api_client, &self.media) {
                    // Report end of playback
                    relm4::spawn({
                        let api_client = api_client.clone();
                        let item_id = media.id.unwrap();
                        async move {
                            api_client
                                .report_playback_stopped(&item_id, position)
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
            }
            VideoPlayerInput::PlayerStateChanged(play_state) => {
                self.set_player_state(play_state);
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
                self.next = next.clone();

                let next = Box::new(next);
                self.controls
                    .emit(VideoPlayerControlsInput::SetNextPreviousEpisodes(
                        Box::new(prev),
                        next.clone(),
                    ));

                if let Some(api_client) = &self.api_client {
                    self.next_up
                        .emit(NextUpInput::SetNextUp((next, api_client.clone())));
                }
            }
        }
    }
}

impl VideoPlayer {
    fn set_player_state(&mut self, new_state: PlayerState) {
        // Seek once playback begins
        // If we seek too early, MPV ignores it
        if matches!(self.player_state, PlayerState::Loading)
            && matches!(new_state, PlayerState::Playing { paused: _ })
        {
            if let Some(playback_position_ticks) = self
                .media
                .as_ref()
                .and_then(|m| m.user_data.clone())
                .and_then(|user_data| user_data.playback_position_ticks)
            {
                let playback_position = ticks_to_seconds(playback_position_ticks);
                self.backend.borrow().seek_to(playback_position as usize);
            }
        }

        match new_state {
            PlayerState::Loading => {
                SCRUBBER_BROKER.read().send(ScrubberInput::Reset);
                PLAY_PAUSE_BROKER.read().send(PlayPauseInput::SetLoading);
                SKIP_FORWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::SetLoading(true));
                SKIP_BACKWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::SetLoading(true));
                self.next_up.emit(NextUpInput::Reset);
            }
            PlayerState::Playing { paused } => {
                SCRUBBER_BROKER.read().send(ScrubberInput::SetPlaying);
                PLAY_PAUSE_BROKER
                    .read()
                    .send(PlayPauseInput::SetPlaying(!paused));
                SKIP_FORWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::SetLoading(false));
                SKIP_BACKWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::SetLoading(false));
            }
            _ => {}
        }

        self.player_state = new_state;
    }

    fn fetch_next_prev(&self, sender: &ComponentSender<Self>, item: &BaseItemDto) {
        if let (Some(api_client), Some(BaseItemKind::Episode), Some(series_id), Some(episode_id)) =
            (&self.api_client, &item.type_, item.series_id, item.id)
        {
            sender.oneshot_command({
                let api_client = api_client.clone();

                async move {
                    let res = match api_client
                        .get_episodes(
                            &GetEpisodesOptionsBuilder::default()
                                .series_id(series_id)
                                .adjacent_to(episode_id)
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
                        [prev, cur] if cur.id.unwrap() == episode_id => {
                            VideoPlayerCommandOutput::LoadedNextPrev((Some(prev.clone()), None))
                        }
                        [cur, next] if cur.id.unwrap() == episode_id => {
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
