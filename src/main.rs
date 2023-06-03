use jellything::app::AppModel;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("cafe.avery.jellything");
    relm4_icons::initialize_icons();
    app.run::<AppModel>(());
}
