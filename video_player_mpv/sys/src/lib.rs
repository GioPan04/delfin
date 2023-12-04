// This file was generated by gir (https://github.com/gtk-rs/gir)
// from build
// from gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type VpmTrackType = c_int;
pub const VPM_TRACK_TYPE_VIDEO: VpmTrackType = 0;
pub const VPM_TRACK_TYPE_AUDIO: VpmTrackType = 1;
pub const VPM_TRACK_TYPE_SUBTITLE: VpmTrackType = 2;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpmTrack {
    pub type_: VpmTrackType,
    pub id: c_int,
    pub title: *mut c_char,
    pub language: *mut c_char,
}

impl ::std::fmt::Debug for VpmTrack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("VpmTrack @ {self:p}"))
            .field("type_", &self.type_)
            .field("id", &self.id)
            .field("title", &self.title)
            .field("language", &self.language)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpmTrackList {
    pub tracks: *mut glib::GList,
}

impl ::std::fmt::Debug for VpmTrackList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("VpmTrackList @ {self:p}"))
            .field("tracks", &self.tracks)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VpmVideoPlayerMpvClass {
    pub parent_class: gtk::GtkGLAreaClass,
}

impl ::std::fmt::Debug for VpmVideoPlayerMpvClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("VpmVideoPlayerMpvClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

// Classes
#[repr(C)]
pub struct VpmVideoPlayerMpv {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for VpmVideoPlayerMpv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("VpmVideoPlayerMpv @ {self:p}"))
            .finish()
    }
}

#[link(name = "video-player-mpv")]
extern "C" {

    //=========================================================================
    // VpmTrack
    //=========================================================================
    pub fn vpm_track_get_type() -> GType;
    pub fn vpm_track_new() -> *mut VpmTrack;
    pub fn vpm_track_copy(t: *mut VpmTrack) -> *mut VpmTrack;
    pub fn vpm_track_free(self_: *mut VpmTrack);
    pub fn vpm_track_id(self_: *const VpmTrack) -> c_int;
    pub fn vpm_track_language(self_: *const VpmTrack) -> *mut c_char;
    pub fn vpm_track_title(self_: *const VpmTrack) -> *mut c_char;
    pub fn vpm_track_type(self_: *const VpmTrack) -> VpmTrackType;

    //=========================================================================
    // VpmTrackList
    //=========================================================================
    pub fn vpm_track_list_get_type() -> GType;
    pub fn vpm_track_list_copy(t: *mut VpmTrackList) -> *mut VpmTrackList;
    pub fn vpm_track_list_free(self_: *mut VpmTrackList);
    pub fn vpm_track_list_is_empty(self_: *const VpmTrackList) -> gboolean;
    pub fn vpm_track_list_len(self_: *const VpmTrackList) -> c_uint;
    pub fn vpm_track_list_track(self_: *const VpmTrackList, index: c_uint) -> *mut VpmTrack;

    //=========================================================================
    // VpmVideoPlayerMpv
    //=========================================================================
    pub fn vpm_video_player_mpv_get_type() -> GType;
    pub fn vpm_video_player_mpv_new() -> *mut gtk::GtkWidget;
    pub fn vpm_video_player_mpv_add_subtitle_track(
        self_: *mut VpmVideoPlayerMpv,
        url: *const c_char,
        title: *const c_char,
    );
    pub fn vpm_video_player_mpv_current_audio_track(self_: *mut VpmVideoPlayerMpv) -> c_int;
    pub fn vpm_video_player_mpv_current_subtitle_track(self_: *mut VpmVideoPlayerMpv) -> c_int;
    pub fn vpm_video_player_mpv_frame_step_backwards(self_: *mut VpmVideoPlayerMpv);
    pub fn vpm_video_player_mpv_frame_step_forwards(self_: *mut VpmVideoPlayerMpv);
    pub fn vpm_video_player_mpv_mute(self_: *mut VpmVideoPlayerMpv) -> bool;
    pub fn vpm_video_player_mpv_pause(self_: *mut VpmVideoPlayerMpv);
    pub fn vpm_video_player_mpv_play(self_: *mut VpmVideoPlayerMpv);
    pub fn vpm_video_player_mpv_play_uri(self_: *mut VpmVideoPlayerMpv, uri: *const c_char);
    pub fn vpm_video_player_mpv_position(self_: *mut VpmVideoPlayerMpv) -> c_double;
    pub fn vpm_video_player_mpv_seek_by(self_: *mut VpmVideoPlayerMpv, seconds: c_int);
    pub fn vpm_video_player_mpv_seek_to(self_: *mut VpmVideoPlayerMpv, seconds: c_uint);
    pub fn vpm_video_player_mpv_set_audio_track(
        self_: *mut VpmVideoPlayerMpv,
        audio_track_id: c_uint,
    );
    pub fn vpm_video_player_mpv_set_mute(self_: *mut VpmVideoPlayerMpv, mute: bool);
    pub fn vpm_video_player_mpv_set_subtitle_background_colour(
        self_: *mut VpmVideoPlayerMpv,
        colour: *mut c_char,
    );
    pub fn vpm_video_player_mpv_set_subtitle_colour(
        self_: *mut VpmVideoPlayerMpv,
        colour: *mut c_char,
    );
    pub fn vpm_video_player_mpv_set_subtitle_font_bold(self_: *mut VpmVideoPlayerMpv, bold: bool);
    pub fn vpm_video_player_mpv_set_subtitle_font_family(
        self_: *mut VpmVideoPlayerMpv,
        family: *mut c_char,
    );
    pub fn vpm_video_player_mpv_set_subtitle_font_italic(
        self_: *mut VpmVideoPlayerMpv,
        italic: bool,
    );
    pub fn vpm_video_player_mpv_set_subtitle_font_size(self_: *mut VpmVideoPlayerMpv, size: c_uint);
    pub fn vpm_video_player_mpv_set_subtitle_position(
        self_: *mut VpmVideoPlayerMpv,
        position: c_uint,
    );
    pub fn vpm_video_player_mpv_set_subtitle_scale(
        self_: *mut VpmVideoPlayerMpv,
        subtitle_scale: c_double,
    );
    pub fn vpm_video_player_mpv_set_subtitle_track(
        self_: *mut VpmVideoPlayerMpv,
        subtitle_track_id: c_uint,
    );
    pub fn vpm_video_player_mpv_set_volume(self_: *mut VpmVideoPlayerMpv, volume: c_double);
    pub fn vpm_video_player_mpv_stop(self_: *mut VpmVideoPlayerMpv);
    pub fn vpm_video_player_mpv_volume(self_: *mut VpmVideoPlayerMpv) -> c_double;

}
