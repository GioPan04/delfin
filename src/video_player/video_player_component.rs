use gst::glib::WeakRef;
use gst::ElementFactory;
use gstplay::PlayVideoOverlayVideoRenderer;
use gtk::prelude::*;
use relm4::gtk::gdk::Paintable;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts};

use crate::api::item::get_stream_url;
use crate::api::latest::LatestMedia;
use crate::config::Server;
use crate::video_player::controls::video_player_controls::{
    VideoPlayerControls, VideoPlayerControlsInit,
};

use super::controls::video_player_controls::VideoPlayerControlsInput;

pub struct VideoPlayer {
    media: LatestMedia,
    player: WeakRef<gstplay::Play>,
    controls: Controller<VideoPlayerControls>,
    show_controls: bool,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    ToggleControls,
    ExitPlayer,
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = (Server, LatestMedia);
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "video-player",

            #[name = "overlay"]
            gtk::Overlay {
                #[name = "video_out"]
                gtk::Picture {
                    set_vexpand: true,
                    add_css_class: "video-out",
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
                        set_title: &model.media.name,
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
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let server = init.0;
        let media = init.1;

        let sink = ElementFactory::make("gtk4paintablesink").build().unwrap();
        let paintable = sink.property::<Paintable>("paintable");
        let renderer = PlayVideoOverlayVideoRenderer::with_sink(&sink);
        let player = gstplay::Play::new(Some(renderer));

        let show_controls = true;

        let controls = VideoPlayerControls::builder()
            .launch(VideoPlayerControlsInit {
                player: player.downgrade(),
                default_show_controls: show_controls,
            })
            .detach();

        let model = VideoPlayer {
            media,
            player: player.downgrade(),
            controls,
            show_controls,
        };

        let widgets = view_output!();
        let overlay = &widgets.overlay;
        let video_out = &widgets.video_out;

        video_out.set_paintable(Some(&paintable));

        overlay.add_overlay(model.controls.widget());

        let url = get_stream_url(&server, &model.media.id);
        player.set_uri(Some(&url));
        relm4::spawn(async move {
            player.play();
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            VideoPlayerInput::ToggleControls => {
                self.show_controls = !self.show_controls;
                self.controls
                    .emit(VideoPlayerControlsInput::SetShowControls(
                        self.show_controls,
                    ));
            }
            VideoPlayerInput::ExitPlayer => {
                if let Some(player) = self.player.upgrade() {
                    player.stop();
                }
                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
        }
    }
}
