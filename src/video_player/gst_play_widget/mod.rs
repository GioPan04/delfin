mod imp;

use std::cell::OnceCell;

use gst::glib::clone::Downgrade;
use gst::glib::SignalHandlerId;
use gst::ClockTime;
use gtk::glib;
use gtk::subclass::prelude::*;
use relm4::gtk;

glib::wrapper! {
    pub struct GstVideoPlayer(ObjectSubclass<imp::GstVideoPlayer>)
        @extends gtk::Widget,
        @implements gtk::Accessible;
}

impl Default for GstVideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl GstVideoPlayer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn player(&self) -> OnceCell<gstplay::Play> {
        self.imp().player.clone()
    }

    pub fn play_uri(&self, uri: &str) {
        let imp = self.imp();

        let player = imp.player.get().unwrap();
        player.set_uri(Some(uri));
        player.play();
    }

    pub fn play(&self) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.play();
    }

    pub fn pause(&self) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.pause();
    }

    pub fn stop(&self) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.stop();
    }

    pub fn seek(&self, seconds: usize) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        let time = ClockTime::from_seconds(seconds as u64);
        player.seek(time);
    }

    pub fn is_muted(&self) -> bool {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.is_muted()
    }

    pub fn set_mute(&self, muted: bool) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.set_mute(muted);
    }

    pub fn set_volume(&self, volume: f64) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.set_volume(volume);
    }

    pub fn position(&self) -> Option<ClockTime> {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.position()
    }

    pub fn connect_end_of_stream<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn() + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_end_of_stream(move |_| {
            callback();
        })
    }

    pub fn connect_buffering<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(i32) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_buffering(move |_, progress| {
            callback(progress);
        })
    }

    pub fn connect_position_updated<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(gst::ClockTime, gst::ClockTime) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();
        let player = imp.player.get().unwrap().downgrade();

        signal_adapter.connect_position_updated(move |_, position| {
            let player = player.upgrade().unwrap();
            if let (Some(position), Some(duration)) = (position, player.duration()) {
                callback(position, duration);
            }
        })
    }

    pub fn connect_mute_changed<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(bool) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_mute_changed(move |_, muted| {
            callback(muted);
        })
    }

    pub fn connect_volume_changed<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(f64) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_volume_changed(move |_, volume| {
            callback(volume);
        })
    }
}
