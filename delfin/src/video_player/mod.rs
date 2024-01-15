pub mod backends;
mod controls;
mod keybindings;
mod mpris;
mod next_up;
mod session;
mod skip_intro;

use crate::config::video_player_config::{VideoPlayerConfig, VideoPlayerOnLeftClick};
use crate::utils::inhibit::InhibitCookie;
use crate::video_player::keybindings::keybindings_controller;
use std::cell::RefCell;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;
use std::time::Duration;

use adw::prelude::*;
use gtk::gdk;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::component::{AsyncComponent, AsyncComponentController, AsyncController};
use relm4::{gtk, ComponentParts};
use relm4::{prelude::*, MessageBroker};

use crate::app::{AppInput, APP_BROKER};
use crate::globals::CONFIG;
use crate::jellyfin_api::api::shows::GetEpisodesOptionsBuilder;
use crate::jellyfin_api::api_client::ApiClient;
use crate::library::LIBRARY_REFRESH_QUEUED;
use crate::media_details::MEDIA_DETAILS_REFRESH_QUEUED;
use crate::tr;
use crate::utils::bif::Thumbnail;
use crate::utils::debounce::Debounce;
use crate::utils::item_name::ItemName;
use crate::utils::ticks::ticks_to_seconds;
use crate::video_player::controls::skip_forwards_backwards::{
    SkipForwardsBackwardsInput, SKIP_BACKWARDS_BROKER, SKIP_FORWARDS_BROKER,
};
use crate::video_player::controls::VideoPlayerControlsInit;
use crate::video_player::next_up::{NextUp, NEXT_UP_VISIBILE};

use self::backends::{PlayerState, VideoPlayerBackend};
use self::controls::fullscreen::{FullscreenInput, FULLSCREEN_BROKER};
use self::controls::play_pause::{PlayPauseInput, PLAY_PAUSE_BROKER};
use self::controls::scrubber::{ScrubberInput, SCRUBBER_BROKER};
use self::controls::volume::{VolumeInput, VOLUME_BROKER};
use self::controls::{VideoPlayerControls, VideoPlayerControlsInput};
use self::mpris::MprisPlaybackReporter;
use self::next_up::NextUpInput;
use self::session::SessionPlaybackReporter;
use self::skip_intro::{SkipIntro, SkipIntroInput};

// How long the cursor has to be still before it's hidden
const CURSOR_HIDE_TIMEOUT: Duration = Duration::from_secs(3);

pub struct VideoPlayer {
    backend: Arc<RefCell<dyn VideoPlayerBackend>>,
    media: Option<BaseItemDto>,
    api_client: Option<Arc<ApiClient>>,
    hiding: Arc<AtomicBool>,

    show_controls: bool,
    show_controls_locked: bool,
    revealer_reveal_child: bool,
    session_playback_reporter: SessionPlaybackReporter,
    mpris_playback_reporter: Option<MprisPlaybackReporter>,
    inhibit_cookie: Option<InhibitCookie>,
    player_state: PlayerState,
    next: Option<BaseItemDto>,

    cursor: Option<gdk::Cursor>,
    cursor_debounce: Debounce,
    last_mouse_position: (f64, f64),

    controls: Controller<VideoPlayerControls>,
    next_up: Controller<NextUp>,
    skip_intro: AsyncController<SkipIntro>,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    ConfigUpdated(VideoPlayerConfig),
    Toast(String),
    PlayVideo(Arc<ApiClient>, Box<BaseItemDto>),
    SetShowControls { show: bool, locked: bool },
    ToggleControls,
    EndOfStream,
    StopPlayer,
    PlayerStateChanged(PlayerState),
    SetRevealerRevealChild(bool),
    MouseMove(f64, f64),
    MouseHide,
    MouseClick(i32),
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[derive(Debug)]
pub enum VideoPlayerCommandOutput {
    LoadedNextPrev((Option<BaseItemDto>, Option<BaseItemDto>)),
    LoadedTrickplay(Option<Vec<Thumbnail>>),
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = ();
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = VideoPlayerCommandOutput;

    view! {
        adw::NavigationPage {
            add_css_class: "video-player-page",
            set_focusable: true,
            add_controller = keybindings_controller(),

            #[watch]
            set_cursor: model.cursor.as_ref(),
            add_controller = gtk::EventControllerMotion {
                connect_motion[sender] => move |_, x, y| {
                    sender.input(VideoPlayerInput::MouseMove(x, y));
                },
            },

            #[watch]
            set_title: &model.media.as_ref()
                .and_then(|media| media
                    .series_and_episode()
                    .or(media.name.clone()))
                .unwrap_or(tr!("app-name").to_string()),

            #[wrap(Some)]
            #[name = "toaster"]
            set_child = &adw::ToastOverlay {
                add_css_class: "video-player",

                gtk::Overlay {
                    #[local_ref]
                    video_player -> gtk::Widget {
                        add_controller = gtk::GestureClick {
                            connect_released[sender] => move |_, n_press, _, _| {
                                sender.input(VideoPlayerInput::MouseClick(n_press));
                            },
                        },
                    },

                    #[name = "revealer"]
                    add_overlay = &gtk::Revealer {
                        #[watch]
                        set_visible: model.show_controls || model.revealer_reveal_child,
                        #[watch]
                        set_reveal_child: model.show_controls,
                        set_transition_type: gtk::RevealerTransitionType::Crossfade,
                        set_valign: gtk::Align::Start,

                        #[wrap(Some)]
                        set_child = &adw::HeaderBar {
                            add_css_class: "osd",
                        },
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

                    add_overlay = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_halign: gtk::Align::Fill,
                        set_valign: gtk::Align::End,
                        set_margin_start: 24,
                        set_margin_end: 24,
                        set_margin_bottom: 24,

                        append = model.skip_intro.widget(),
                        append = model.controls.widget(),
                    },

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
            .forward(sender.input_sender(), |msg| match msg {
                controls::VideoPlayerControlsOutput::ShowControls => {
                    VideoPlayerInput::ToggleControls
                }
            });

        let next_up = NextUp::builder().launch(backend.clone()).detach();
        let skip_intro = SkipIntro::builder().launch(backend.clone()).detach();

        let cursor_debounce = Debounce::new(
            CURSOR_HIDE_TIMEOUT,
            Box::new({
                let sender = sender.clone();
                move || {
                    sender.input(VideoPlayerInput::MouseHide);
                }
            }),
        );

        let model = VideoPlayer {
            backend,
            media: None,
            api_client: None,
            hiding: Arc::new(AtomicBool::new(false)),

            show_controls,
            show_controls_locked: false,
            revealer_reveal_child: show_controls,
            session_playback_reporter: SessionPlaybackReporter::default(),
            mpris_playback_reporter: None,
            inhibit_cookie: None,
            player_state: PlayerState::Loading,
            next: None,

            cursor: None,
            cursor_debounce,
            last_mouse_position: (0.0, 0.0),

            controls,
            next_up,
            skip_intro,
        };

        model.configure_player(&CONFIG.read().video_player);
        model.subscribe_to_config(&sender);

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
        let revealer = &widgets.revealer;

        revealer.connect_child_revealed_notify({
            let sender = sender.clone();
            move |revealer| {
                sender.input(VideoPlayerInput::SetRevealerRevealChild(
                    revealer.is_child_revealed(),
                ));
            }
        });

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
            VideoPlayerInput::ConfigUpdated(video_player_config) => {
                self.configure_player(&video_player_config);
            }
            VideoPlayerInput::Toast(message) => {
                let toast = adw::Toast::new(&message);
                widgets.toaster.add_toast(toast);
            }
            VideoPlayerInput::PlayVideo(api_client, item) => {
                self.inhibit_cookie = InhibitCookie::new().ok();

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

                self.controls.emit(VideoPlayerControlsInput::SetPlaying {
                    api_client: api_client.clone(),
                    item: Box::new(*item.clone()),
                });

                if let Some(item_id) = item.id {
                    // Report start of playback
                    relm4::spawn({
                        let api_client = api_client.clone();
                        async move {
                            api_client
                                .report_playback_started(&item_id.to_string())
                                .await
                                .unwrap();
                        }
                    });

                    // Load intro skipper
                    self.skip_intro
                        .emit(SkipIntroInput::Load(item_id, api_client.clone()));
                }

                // Starts a background task that continuously reports playback progress
                self.session_playback_reporter.start(
                    api_client.clone(),
                    &item.id.unwrap(),
                    self.backend.clone(),
                );

                self.mpris_playback_reporter = Some(MprisPlaybackReporter::new(
                    api_client.clone(),
                    *item.clone(),
                    self.backend.clone(),
                ));

                self.api_client = Some(api_client);

                self.fetch_next_prev(&sender, &item);
                self.fetch_trickplay(&sender, &item);
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
                self.inhibit_cookie = None;

                self.hiding.store(true, atomic::Ordering::Relaxed);

                self.backend.borrow_mut().stop();

                let position = self.backend.borrow().position();

                // Report end of playback
                // (don't report if we're still loading)
                if !matches!(self.player_state, PlayerState::Loading) {
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
                                *LIBRARY_REFRESH_QUEUED.write() = true;
                                *MEDIA_DETAILS_REFRESH_QUEUED.write() = true;
                            }
                        });

                        self.api_client = None;
                        self.media = None;
                    }
                }

                // Stop background playback progress reporter
                self.session_playback_reporter.stop(self.backend.clone());

                self.mpris_playback_reporter = None;

                FULLSCREEN_BROKER.send(FullscreenInput::ExitFullscreen);
            }
            VideoPlayerInput::PlayerStateChanged(play_state) => {
                self.set_player_state(play_state);
            }
            VideoPlayerInput::SetRevealerRevealChild(reveal) => {
                self.revealer_reveal_child = reveal;
            }
            VideoPlayerInput::MouseMove(x, y) => {
                // For some reason mouse move events were getting fired even though the mouse
                // position didn't change. For now, store the last position so we can check if it
                // actually moved.
                if (x, y) != self.last_mouse_position {
                    self.cursor = None;
                    self.cursor_debounce.debounce();
                    self.last_mouse_position = (x, y);

                    if matches!(
                        CONFIG.read().video_player.on_left_click,
                        VideoPlayerOnLeftClick::PlayPause
                    ) && !self.show_controls_locked
                    {
                        sender.input(VideoPlayerInput::SetShowControls {
                            show: true,
                            locked: self.show_controls_locked,
                        });
                    }
                }
            }
            VideoPlayerInput::MouseHide => {
                self.cursor = gdk::Cursor::from_name("none", None);

                if let VideoPlayerOnLeftClick::PlayPause = CONFIG.read().video_player.on_left_click
                {
                    sender.input(VideoPlayerInput::SetShowControls {
                        show: false,
                        locked: self.show_controls_locked,
                    });
                }
            }
            VideoPlayerInput::MouseClick(n_press) => match n_press {
                2 => FULLSCREEN_BROKER.send(FullscreenInput::ToggleFullscreen),
                _ => match CONFIG.read().video_player.on_left_click {
                    VideoPlayerOnLeftClick::PlayPause => {
                        PLAY_PAUSE_BROKER.send(PlayPauseInput::TogglePlaying);
                    }
                    VideoPlayerOnLeftClick::ToggleControls => {
                        sender.input(VideoPlayerInput::ToggleControls);
                    }
                },
            },
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
            VideoPlayerCommandOutput::LoadedTrickplay(thumbnails) => {
                SCRUBBER_BROKER
                    .read()
                    .send(ScrubberInput::LoadedThumbnails(thumbnails));
            }
        }
    }
}

impl VideoPlayer {
    fn subscribe_to_config(&self, sender: &ComponentSender<Self>) {
        CONFIG.subscribe(sender.input_sender(), |config| {
            VideoPlayerInput::ConfigUpdated(config.video_player.clone())
        });
    }

    fn configure_player(&self, video_player_config: &VideoPlayerConfig) {
        let player = self.backend.borrow();
        player.set_subtitle_scale(video_player_config.subtitle_scale);
        player.set_subtitle_colour(
            video_player_config
                .subtitle_colour
                .clone()
                .try_into()
                .unwrap_or_else(|_| {
                    panic!(
                        "Error setting subtitle colour: {}",
                        video_player_config.subtitle_colour
                    )
                }),
        );
        player.set_subtitle_background_colour(
            video_player_config
                .subtitle_background_colour
                .clone()
                .try_into()
                .unwrap_or_else(|_| {
                    panic!(
                        "Error setting subtitle background colour: {}",
                        video_player_config.subtitle_background_colour
                    )
                }),
        );
        player.set_subtitle_position(video_player_config.subtitle_position);
        player.set_subtitle_font(&video_player_config.subtitle_font);
    }

    fn set_player_state(&mut self, new_state: PlayerState) {
        if matches!(self.player_state, PlayerState::Loading)
            && matches!(new_state, PlayerState::Playing { paused: _ })
        {
            // Seek once playback begins
            // If we seek too early, MPV ignores it
            if let Some(playback_position_ticks) = self
                .media
                .as_ref()
                .and_then(|m| m.user_data.clone())
                .and_then(|user_data| user_data.playback_position_ticks)
            {
                let playback_position = ticks_to_seconds(playback_position_ticks);
                self.backend.borrow().seek_to(playback_position as usize);
            }

            // Load volume settings once playback starts
            VOLUME_BROKER.send(VolumeInput::LoadSettings);
        }

        match new_state {
            PlayerState::Loading => {
                SCRUBBER_BROKER.send(ScrubberInput::Reset);
                PLAY_PAUSE_BROKER.send(PlayPauseInput::SetLoading);
                SKIP_FORWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::SetLoading(true));
                SKIP_BACKWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::SetLoading(true));
                self.next_up.emit(NextUpInput::Reset);
            }
            PlayerState::Playing { paused } => {
                SCRUBBER_BROKER.send(ScrubberInput::SetPlaying);
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

    fn fetch_trickplay(&self, sender: &ComponentSender<Self>, item: &BaseItemDto) {
        let (api_client, id) = match (&self.api_client, item.id) {
            (Some(api_client), Some(id)) => (api_client, id),
            _ => return,
        };

        sender.oneshot_command({
            let api_client = api_client.clone();
            async move {
                if !CONFIG.read().video_player.jellyscrub {
                    return VideoPlayerCommandOutput::LoadedTrickplay(None);
                }

                let manifest = match api_client.get_trickplay_manifest(&id).await {
                    Ok(Some(manifest)) if !manifest.width_resolutions.is_empty() => manifest,
                    Ok(None) | Ok(Some(_)) => {
                        return VideoPlayerCommandOutput::LoadedTrickplay(None);
                    }
                    Err(err) => {
                        println!("Error fetching trickplay manifest: {err}");
                        return VideoPlayerCommandOutput::LoadedTrickplay(None);
                    }
                };

                let width = match manifest.width_resolutions.iter().max() {
                    Some(width) => width,
                    _ => return VideoPlayerCommandOutput::LoadedTrickplay(None),
                };

                let thumbnails = match api_client.get_trickplay_thumbnails(&id, *width).await {
                    Ok(Some(thumbnails)) => thumbnails,
                    Ok(None) => {
                        return VideoPlayerCommandOutput::LoadedTrickplay(None);
                    }
                    Err(err) => {
                        println!("Error fetching trickplay thumbnails: {err}");
                        return VideoPlayerCommandOutput::LoadedTrickplay(None);
                    }
                };

                VideoPlayerCommandOutput::LoadedTrickplay(Some(thumbnails))
            }
        });
    }
}

pub static VIDEO_PLAYER_BROKER: MessageBroker<VideoPlayerInput> = MessageBroker::new();
