use std::cell::OnceCell;

use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts};

use crate::api::item::get_stream_url;
use crate::api::latest::LatestMedia;
use crate::config::Server;
use crate::utils::ticks::ticks_to_seconds;
use crate::video_player::controls::video_player_controls::{
    VideoPlayerControls, VideoPlayerControlsInit,
};
use crate::video_player::gst_play_widget::GstVideoPlayer;

use super::controls::video_player_controls::VideoPlayerControlsInput;

pub struct VideoPlayer {
    controls: OnceCell<Controller<VideoPlayerControls>>,
    media: Option<LatestMedia>,
    show_controls: bool,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    PlayVideo(Server, LatestMedia),
    ToggleControls,
    ExitPlayer,
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = ();
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
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
            },
        }

    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let show_controls = true;

        let controls = OnceCell::new();

        let model = VideoPlayer {
            media: None,
            controls,
            show_controls,
        };

        let video_player = GstVideoPlayer::new();

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
            VideoPlayerInput::PlayVideo(server, media) => {
                self.media = Some(media.clone());
                let url = get_stream_url(&server, &media.id);
                widgets.video_player.play_uri(&url);
                let playback_position = ticks_to_seconds(media.user_data.playback_position_ticks);
                widgets.video_player.seek(playback_position);
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
                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
        }

        self.update_view(widgets, sender);
    }
}
