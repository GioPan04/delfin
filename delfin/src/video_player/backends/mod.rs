use std::fmt;

use relm4::gtk;
use uuid::Uuid;

#[cfg(feature = "gst")]
pub mod gst;

pub mod mpv;

#[derive(Clone, Debug)]
pub struct SubtitleTrack {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct AudioTrack {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Copy, Debug)]
pub enum PlayerState {
    Loading,
    Playing { paused: bool },
    Buffering,
}

type PlayerStateChangedCallback = Box<dyn Fn(PlayerState)>;

pub trait VideoPlayerBackend: fmt::Debug {
    /// Returns widget that renders the video.
    fn widget(&self) -> &gtk::Widget;

    /// Get notified when the player state changes.
    fn connect_player_state_changed(&mut self, callback: PlayerStateChangedCallback) -> Uuid;

    /// Disconnect a player state changed callback.
    fn disconnect_player_state_changed(&mut self, id: Uuid);

    /// Stream video from the given URI.
    fn play_uri(&mut self, uri: &str);

    /// Play the current video.
    fn play(&self);

    /// Pause the current video.
    fn pause(&self);

    /// Stops the current video and performs clean up.
    fn stop(&mut self);

    /// Seek to timestamp in seconds.
    fn seek_to(&self, seconds: usize);

    /// Seek by seconds relative to current timestamp.
    fn seek_by(&self, seconds: isize);

    /// Check if the player is currently muted.
    fn muted(&self) -> bool;

    /// Set if audio is muted.
    fn set_muted(&self, muted: bool);

    /// Get the current volume level.
    fn volume(&self) -> f64;

    /// Set volume in percentage (0 - 1).
    fn set_volume(&self, volume: f64);

    /// Get current video position in seconds.
    fn position(&self) -> usize;

    // Disconnects the signal handler for the given ID.
    // TODO: All connect methods should return Uuid, and probably unify this with player state
    // callbacks
    fn disconnect_signal_handler(&mut self, id: &Uuid);

    /// Get the current subtitle track ID.
    fn current_subtitle_track(&self) -> Option<usize>;

    /// Set the current subtitle track.
    fn set_subtitle_track(&self, subtitle_track_id: Option<usize>);

    /// Get the current audio track ID.
    fn current_audio_track(&self) -> Option<usize>;

    /// Set the current audio track.
    fn set_audio_track(&self, audio_track_id: Option<usize>);

    /// Get notified when video player reaches the end of the current video.
    fn connect_end_of_stream(&mut self, callback: Box<dyn Fn() + Send + 'static>);

    /// Get notified when the playback position changes.
    fn connect_position_updated(
        &mut self,
        callback: Box<dyn Fn(usize) + Send + Sync + 'static>,
    ) -> Uuid;

    /// Get notified when the media duration changes.
    fn connect_duration_updated(&mut self, callback: Box<dyn Fn(usize) + Send + Sync + 'static>);

    /// Get notified when the player is muted or unmuted.
    fn connect_mute_updated(&mut self, callback: Box<dyn Fn(bool) + Send + Sync + 'static>);

    /// Get notified when the player volume changes.
    fn connect_volume_updated(&mut self, callback: Box<dyn Fn(f64) + Send + Sync + 'static>);

    /// Get notified when the list of avilable subtitle tracks changes.
    fn connect_subtitle_tracks_updated(
        &mut self,
        callback: Box<dyn Fn(Vec<SubtitleTrack>) + Send + Sync + 'static>,
    );

    /// Get notified when the list of avilable audio tracks changes.
    fn connect_audio_tracks_updated(
        &mut self,
        callback: Box<dyn Fn(Vec<AudioTrack>) + Send + Sync + 'static>,
    );
}
