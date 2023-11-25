use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use delfin::{
    app::{App, APP_BROKER},
    meson_config::{APP_ID, BUILDDIR, RESOURCES_FILE},
};
use gtk::gio;
use relm4::{gtk, RelmApp};

fn main() {
    env_logger::init();

    #[cfg(feature = "gst")]
    {
        video_player_gst::init_gst();
    }

    load_resources().expect("Error loading resources");

    let app = RelmApp::new(APP_ID);
    relm4_icons::initialize_icons();
    load_css();
    app.with_broker(&APP_BROKER).run::<App>(());
}

fn load_css() {
    let css = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
    relm4::set_global_css(css);
}

fn load_resources() -> Result<()> {
    let res = match gio::Resource::load(RESOURCES_FILE) {
        Ok(res) => res,
        Err(_) if cfg!(debug_assertions) => {
            gio::Resource::load(PathBuf::from(BUILDDIR).join("data/resources.gresource"))
                .context("Could not load fallback gresource file")?
        }
        Err(_) => bail!("Could not load gresource file"),
    };

    gio::resources_register(&res);
    Ok(())
}
