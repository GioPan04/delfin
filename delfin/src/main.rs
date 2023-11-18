use delfin::{
    app::{App, APP_BROKER},
    meson_config::APP_ID,
};
use relm4::RelmApp;

fn main() {
    env_logger::init();

    #[cfg(feature = "gst")]
    {
        video_player_gst::init_gst();
    }

    let app = RelmApp::new(APP_ID);

    relm4_icons::initialize_icons();
    load_css();
    app.with_broker(&APP_BROKER).run::<App>(());
}

fn load_css() {
    // TODO: move this to a dev dependency once this is done at compile time
    use regex::Regex;

    // TODO: compile sass in build.rs
    let css = grass::from_string(include_str!("app.scss"), &grass::Options::default()).unwrap();

    // Output GTK's at-rules. Sass doesn't support custom at-rules, so we wrap
    // them in a string and unwrap them after Sass has done it's thing.

    // GTK colour expressions - these are similar to Sass's colour functions,
    // but they can operate on GTK colour variables.
    let re_gtk_expressions = Regex::new(r#""--gtk-(.+)""#).unwrap();
    // GTK colour variables
    let re_gtk_colours = Regex::new(r#""(@.+)""#).unwrap();

    let css = re_gtk_expressions.replace_all(&css, "$1");
    let css = re_gtk_colours.replace_all(&css, "$1");

    relm4::set_global_css(&css);
}
