use std::sync::{Arc, RwLock};

use jellything::{
    app::{App, APP_BROKER},
    config::Config,
};
use relm4::RelmApp;

fn main() {
    env_logger::init();

    gst::init().expect("Error initializing GStreamer");
    gstgtk4::plugin_register_static().expect("Error registering GST GTK4 plugin");

    let config = Config::new().expect("Error creating config.");

    let app = RelmApp::new("cafe.avery.jellything");
    relm4_icons::initialize_icons();
    load_css();
    app.with_broker(&APP_BROKER)
        .run::<App>(Arc::new(RwLock::new(config)));
}

fn load_css() {
    // TODO: compile sass in build.rs
    let css = grass::from_path("src/app.scss", &grass::Options::default()).unwrap();
    relm4::set_global_css(&css);
}
