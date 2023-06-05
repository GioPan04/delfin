use gst::traits::ElementExt;
use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts, SimpleComponent};

use crate::video_player::gtksink::create_gtk_sink;
use crate::video_player::pipeline::create_pipeline;

pub struct VideoPlayer {
    show_controls: bool,
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    ToggleControls,
}

#[relm4::component(pub)]
impl SimpleComponent for VideoPlayer {
    type Init = String;
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
                        set_title: "Jellything",
                    },
                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous",
                        connect_clicked[sender] => move |_| {
                            sender.output(VideoPlayerOutput::NavigateBack).unwrap();
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

                    gtk::Scale {
                        set_range: (0.0, 100.0),
                        set_value: 25.0,
                        set_hexpand: true,
                    },

                    gtk::Label {
                        set_label: "4:20/69:42",
                        add_css_class: "duration",
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
        let model = VideoPlayer {
            show_controls: false,
        };

        let widgets = view_output!();
        let video_out = &widgets.video_out;

        let (sink, paintable) = create_gtk_sink();
        video_out.set_paintable(Some(&paintable));

        let pipeline = create_pipeline(&init, Box::new(sink));
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set pipeline to Playing state");

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            VideoPlayerInput::ToggleControls => self.show_controls = !self.show_controls,
        }
    }
}
