use std::cell::OnceCell;

use gst::prelude::*;
use gstplay::PlaySignalAdapter;

use gst::ElementFactory;
use gstplay::PlayVideoOverlayVideoRenderer;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use relm4::gtk;
use relm4::gtk::gdk;

#[derive(Debug, Default)]
pub struct GstVideoPlayer {
    pub player: OnceCell<gstplay::Play>,
    pub signal_adapter: OnceCell<PlaySignalAdapter>,
    picture: OnceCell<gtk::Picture>,
}

#[glib::object_subclass]
impl ObjectSubclass for GstVideoPlayer {
    const NAME: &'static str = "GstVideoPlayer";
    type Type = super::GstVideoPlayer;
    type ParentType = gtk::Widget;

    fn class_init(class: &mut Self::Class) {
        class.set_layout_manager_type::<gtk::BoxLayout>();
    }
}

impl ObjectImpl for GstVideoPlayer {
    fn constructed(&self) {
        load_css();

        self.parent_constructed();
        let obj = self.obj();
        obj.add_css_class("gst-video-player");
        obj.set_hexpand(true);
        obj.set_vexpand(true);

        let sink = ElementFactory::make("gtk4paintablesink").build().unwrap();
        let paintable = sink.property::<gdk::Paintable>("paintable");
        let renderer = PlayVideoOverlayVideoRenderer::with_sink(&sink);

        let player = gstplay::Play::new(Some(renderer));
        let mut player_config = player.config();
        player_config.set_position_update_interval(250);
        player
            .set_config(player_config)
            .expect("Failed to set player config.");

        let signal_adapter = PlaySignalAdapter::new(&player);

        let picture = gtk::Picture::new();
        picture.add_css_class("gst-video-player__video-out");
        picture.set_hexpand(true);
        picture.set_vexpand(true);
        picture.set_parent(&*obj);
        picture.set_paintable(Some(&paintable));

        self.picture.set(picture).unwrap();
        self.signal_adapter.set(signal_adapter).unwrap();
        self.player.set(player).unwrap();
    }

    fn dispose(&self) {
        self.player.get().unwrap().stop();
        self.picture.get().unwrap().unparent();
    }
}

impl WidgetImpl for GstVideoPlayer {}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        r#"
        .gst-video-player {
            background: black;
        }

        .gst-video-player__video-out {
            border-radius: 12px;
        }
    "#,
    );

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
