mod imp;

use std::cell::OnceCell;

use gst::glib::{Error, SignalHandlerId};
use gst::{ClockTime, Structure};
use gstplay::{PlayAudioInfo, PlayMediaInfo, PlaySubtitleInfo};
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

    pub fn seek(&self, time: ClockTime) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
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

    pub fn current_subtitle_track(&self) -> Option<PlaySubtitleInfo> {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.current_subtitle_track()
    }

    pub fn set_subtitle_track_enabled(&self, enabled: bool) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.set_subtitle_track_enabled(enabled);
    }

    pub fn set_subtitle_track(&self, stream_index: i32) -> Result<(), glib::error::BoolError> {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.set_subtitle_track(stream_index)
    }

    pub fn current_audio_track(&self) -> Option<PlayAudioInfo> {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.current_audio_track()
    }

    pub fn set_audio_track_enabled(&self, enabled: bool) {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.set_audio_track_enabled(enabled);
    }

    pub fn set_audio_track(&self, stream_index: i32) -> Result<(), glib::error::BoolError> {
        let imp = self.imp();
        let player = imp.player.get().unwrap();
        player.set_audio_track(stream_index)
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

    pub fn connect_error<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(&Error, Option<&Structure>) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_error(move |_, error, structure| {
            callback(error, structure);
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
        F: Fn(ClockTime) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_position_updated(move |_, position| {
            if let Some(position) = position {
                callback(position);
            }
        })
    }

    pub fn connect_duration_changed<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(ClockTime) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_duration_changed(move |_, duration| {
            if let Some(duration) = duration {
                callback(duration)
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

    pub fn connect_media_info_updated<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(&PlayMediaInfo) + Send + 'static,
    {
        let imp = self.imp();

        let signal_adapter = imp.signal_adapter.get().unwrap();

        signal_adapter.connect_media_info_updated(move |_, media_info| {
            callback(media_info);
        })
    }
}
