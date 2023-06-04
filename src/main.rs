use std::sync::{Arc, RwLock};

use jellything::{app::App, config::Config};
use relm4::RelmApp;

fn main() {
    env_logger::init();

    gst::init().expect("Error initializing GStreamer");
    gstgtk4::plugin_register_static().expect("Error registering GST GTK4 plugin");

    let config = Config::new().expect("Error creating config.");
    println!("Config: {:#?}", config);

    let app = RelmApp::new("cafe.avery.jellything");
    relm4_icons::initialize_icons();
    app.run::<App>(Arc::new(RwLock::new(config)));
}
