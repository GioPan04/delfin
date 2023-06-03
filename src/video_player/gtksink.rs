use gst::{prelude::*, Element, ElementFactory};
use relm4::gtk::gdk::Paintable;

pub fn create_gtk_sink() -> (Element, Paintable) {
    let sink = ElementFactory::make("gtk4paintablesink").build().unwrap();
    let paintable = sink.property::<Paintable>("paintable");
    (sink, paintable)
}
