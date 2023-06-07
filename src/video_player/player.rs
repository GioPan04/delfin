use gst::{prelude::*, Element, ElementFactory};
use gstplay::{Play, PlayVideoOverlayVideoRenderer};
use relm4::gtk::gdk::Paintable;

pub fn create_player() -> (Play, Paintable) {
    let (sink, paintable) = create_gtk_sink();
    let renderer = PlayVideoOverlayVideoRenderer::with_sink(&sink);
    let player = Play::new(Some(renderer));
    (player, paintable)
}

fn create_gtk_sink() -> (Element, Paintable) {
    let sink = ElementFactory::make("gtk4paintablesink").build().unwrap();
    let paintable = sink.property::<Paintable>("paintable");
    (sink, paintable)
}
