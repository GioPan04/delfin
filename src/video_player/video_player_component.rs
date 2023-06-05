use std::cell::RefCell;
use std::time::Duration;

use gst::prelude::ElementExtManual;
use gst::traits::ElementExt;
use gtk::prelude::*;
use relm4::gtk::glib;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts, SimpleComponent};

use crate::api::item::get_stream_url;
use crate::api::latest::LatestMedia;
use crate::config::Server;
use crate::video_player::gtksink::create_gtk_sink;
use crate::video_player::pipeline::create_pipeline;

pub struct VideoPlayer {
    media: LatestMedia,
    pipeline: Option<gst::Pipeline>,
    playback_timeout_id: RefCell<Option<glib::SourceId>>,
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
impl SimpleComponent for VideoPlayer {
    type Init = (Server, LatestMedia);
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "video-player",

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
                    add_css_class: "video-player-header",
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

                add_overlay = &gtk::Box {
                    #[watch]
                    set_visible: model.show_controls,
                    set_valign: gtk::Align::End,
                    add_css_class: "video-player-controls",

                    gtk::Button {
                        set_icon_name: "media-playback-start",
                        add_css_class: "flat",
                        add_css_class: "play-pause",
                    },

                    #[name = "scrubber"]
                    gtk::Scale {
                        set_range: (0.0, 100.0),
                        set_value: 0.0,
                        set_hexpand: true,
                    },

                    #[name = "timestamp"]
                    gtk::Label {
                        set_label: "4:20/69:42",
                        add_css_class: "timestamp",
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

        let mut model = VideoPlayer {
            media,
            pipeline: None,
            playback_timeout_id: RefCell::new(None),
            show_controls: false,
        };

        let widgets = view_output!();
        let video_out = &widgets.video_out;
        let scrubber = &widgets.scrubber;
        let timestamp = &widgets.timestamp;

        let (sink, paintable) = create_gtk_sink();
        video_out.set_paintable(Some(&paintable));

        let url = get_stream_url(&server, &model.media.id);

        let pipeline = create_pipeline(&url, Box::new(sink));
        // TODO: stop pipeline and timeouts when leaving video player
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set pipeline to Playing state");

        {
            let pipeline = pipeline.downgrade();
            let scrubber = scrubber.downgrade();
            let timestamp = timestamp.downgrade();
            model.playback_timeout_id = RefCell::new(Some(glib::timeout_add_local(
                Duration::from_millis(500),
                move || {
                    let pipeline = match pipeline.upgrade() {
                        Some(pipeline) => pipeline,
                        None => return glib::Continue(true),
                    };

                    let (scrubber, timestamp) = match (scrubber.upgrade(), timestamp.upgrade()) {
                        (Some(scrubber), Some(timestamp)) => (scrubber, timestamp),
                        _ => return glib::Continue(true),
                    };

                    let position = pipeline.query_position::<gst::ClockTime>();
                    // TODO: some formats don't have duration, maybe we can default
                    // to the one that Jellyfin gives us in RunTimeTicks
                    let duration = pipeline.query_duration::<gst::ClockTime>();
                    if let (Some(position), Some(duration)) = (position, duration) {
                        scrubber.set_range(0.0, duration.seconds() as f64);
                        scrubber.set_value(position.seconds() as f64);
                        timestamp.set_label(&format!(
                            "{} / {}",
                            position.to_timestamp(),
                            duration.to_timestamp()
                        ));
                        println!("{} / {}", position.to_timestamp(), duration.to_timestamp());
                    }

                    glib::Continue(true)
                },
            )));
        }

        model.pipeline = Some(pipeline);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            VideoPlayerInput::ToggleControls => self.show_controls = !self.show_controls,
            VideoPlayerInput::ExitPlayer => {
                if let Some(playback_timeout_id) = self.playback_timeout_id.borrow_mut().take() {
                    playback_timeout_id.remove();
                }
                if let Some(pipeline) = &self.pipeline {
                    pipeline.set_state(gst::State::Paused).unwrap();
                    pipeline.set_state(gst::State::Ready).unwrap();
                    pipeline.set_state(gst::State::Null).unwrap();
                }
                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
        }
    }
}

trait ToTimestamp {
    fn to_timestamp(self) -> String;
}

impl ToTimestamp for gst::ClockTime {
    fn to_timestamp(self) -> String {
        let minutes = self.seconds() / 60;
        let seconds = self.seconds() % 60;
        format!("{:0>2}:{:0>2}", minutes, seconds)
    }
}
