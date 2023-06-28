use std::cell::OnceCell;
use std::sync::{Arc, RwLock};

use gtk::prelude::*;
use relm4::{gtk, ComponentParts};
use relm4::{prelude::*, JoinHandle};

use crate::config::{Config, Server};
use crate::jellyfin_api::{api::item::get_stream_url, api_client::ApiClient, models::media::Media};
use crate::utils::ticks::ticks_to_seconds;
use crate::video_player::controls::video_player_controls::{
    VideoPlayerControls, VideoPlayerControlsInit,
};
use crate::video_player::gst_play_widget::GstVideoPlayer;

use super::controls::video_player_controls::VideoPlayerControlsInput;
use super::session::start_session_reporting;

pub struct VideoPlayer {
    config: Arc<RwLock<Config>>,
    controls: OnceCell<Controller<VideoPlayerControls>>,
    media: Option<Media>,
    api_client: Option<Arc<ApiClient>>,
    show_controls: bool,
    session_reporting_handle: Option<JoinHandle<()>>,
    buffering: bool,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    Toast(String),
    PlayVideo(Arc<ApiClient>, Server, Media),
    ToggleControls,
    SetBuffering(bool),
    ExitPlayer,
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = Arc<RwLock<Config>>;
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = ();

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
                    set_visible: model.buffering,
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
            buffering: false,
        };

        let video_player = GstVideoPlayer::new();

        video_player.connect_buffering({
            let sender = sender.clone();
            move |progress| {
                sender.input(VideoPlayerInput::SetBuffering(progress != 100));
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

                self.media = Some(media.clone());
                let url = get_stream_url(&server, &media.id);
                video_player.play_uri(&url);

                let playback_position = ticks_to_seconds(media.user_data.playback_position_ticks);
                video_player.seek(playback_position);

                if let Some(controls) = self.controls.get() {
                    controls.emit(VideoPlayerControlsInput::SetPlaying(media.clone()));
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
            }
            VideoPlayerInput::ToggleControls => {
                self.show_controls = !self.show_controls;
                let controls = self.controls.get().unwrap();
                controls.emit(VideoPlayerControlsInput::SetShowControls(
                    self.show_controls,
                ));
            }
            VideoPlayerInput::SetBuffering(buffering) => self.buffering = buffering,
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
        }

        self.update_view(widgets, sender);
    }
}
