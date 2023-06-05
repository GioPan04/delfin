use gst::traits::ElementExt;
use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts, SimpleComponent};

use crate::video_player::gtksink::create_gtk_sink;
use crate::video_player::pipeline::create_pipeline;

pub struct VideoPlayer {}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl SimpleComponent for VideoPlayer {
    type Init = String;
    type Input = ();
    type Output = VideoPlayerOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::HeaderBar {
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

            #[name = "video_out"]
            gtk::Picture {
                set_vexpand: true,
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widgets = view_output!();
        let video_out = &widgets.video_out;

        let (sink, paintable) = create_gtk_sink();
        video_out.set_paintable(Some(&paintable));

        let pipeline = create_pipeline(&init, Box::new(sink));
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set pipeline to Playing state");

        let model = VideoPlayer {};
        ComponentParts { model, widgets }
    }
}
