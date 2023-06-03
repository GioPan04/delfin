use jellything::app::App;
use relm4::RelmApp;

fn main() {
    gst::init().expect("Error initializing GStreamer");
    gstgtk4::plugin_register_static().expect("Error registering GST GTK4 plugin");

    let app = RelmApp::new("cafe.avery.jellything");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}
