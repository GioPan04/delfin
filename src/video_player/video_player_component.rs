use gst::traits::ElementExt;
use relm4::prelude::*;
use relm4::{ComponentParts, SimpleComponent};

use crate::video_player::gtksink::create_gtk_sink;
use crate::video_player::pipeline::create_pipeline;

pub struct VideoPlayer {}

#[relm4::component(pub)]
impl SimpleComponent for VideoPlayer {
    type Init = String;
    type Input = ();
    type Output = ();

    view! {
        #[name = "video_out"]
        gtk::Picture {}
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: relm4::ComponentSender<Self>,
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
