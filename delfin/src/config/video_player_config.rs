use std::{cell::RefCell, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    utils::round::round_one_place,
    video_player::backends::{mpv::VideoPlayerBackendMpv, VideoPlayerBackend},
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum VideoPlayerBackendPreference {
    Mpv,
    Gst,
}

impl From<VideoPlayerBackendPreference> for Arc<RefCell<dyn VideoPlayerBackend>> {
    fn from(val: VideoPlayerBackendPreference) -> Self {
        match val {
            VideoPlayerBackendPreference::Mpv => Arc::<RefCell<VideoPlayerBackendMpv>>::default(),
            VideoPlayerBackendPreference::Gst => {
                #[cfg(feature = "gst")]
                {
                    Arc::<RefCell<crate::video_player::backends::gst::VideoPlayerBackendGst>>::default()
                }

                #[cfg(not(feature = "gst"))]
                {
                    println!("GStreamer backend not available, falling back to MPV backend");
                    Arc::<RefCell<VideoPlayerBackendMpv>>::default()
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct VideoPlayerConfig {
    pub position_update_frequency: usize,
    pub volume: f64,
    pub muted: bool,

    pub skip_backwards_amount: VideoPlayerSkipAmount,
    pub skip_forwards_amount: VideoPlayerSkipAmount,
    pub on_left_click: VideoPlayerOnLeftClick,

    #[serde(serialize_with = "round_one_place")]
    pub subtitle_scale: f64,

    pub intro_skipper: bool,
    pub intro_skipper_auto_skip: bool,
    pub jellyscrub: bool,

    pub backend: VideoPlayerBackendPreference,
    pub hls_playback: bool,
}

impl Default for VideoPlayerConfig {
    fn default() -> Self {
        Self {
            position_update_frequency: 10,
            volume: 1.0,
            muted: false,

            skip_backwards_amount: VideoPlayerSkipAmount::Ten,
            skip_forwards_amount: VideoPlayerSkipAmount::Thirty,
            on_left_click: VideoPlayerOnLeftClick::PlayPause,

            subtitle_scale: 1.0,

            backend: VideoPlayerBackendPreference::Mpv,
            hls_playback: false,
            intro_skipper: true,
            intro_skipper_auto_skip: true,
            jellyscrub: true,
        }
    }
}

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum VideoPlayerSkipAmount {
    Ten = 10,
    Thirty = 30,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum VideoPlayerOnLeftClick {
    PlayPause,
    ToggleControls,
}
