use std::{cell::RefCell, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    utils::round::round_one_place,
    video_player::backends::{
        mpv::VideoPlayerBackendMpv, VideoPlayerBackend, VideoPlayerSubtitleFont,
    },
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct VideoPlayerConfig {
    pub position_update_frequency: usize,
    pub volume: f64,
    pub muted: bool,

    pub skip_backwards_amount: VideoPlayerSkipAmount,
    pub skip_forwards_amount: VideoPlayerSkipAmount,
    pub on_left_click: VideoPlayerOnLeftClick,
    pub duration_display: DurationDisplay,

    #[serde(serialize_with = "round_one_place")]
    pub subtitle_scale: f64,
    pub subtitle_colour: String,
    pub subtitle_background_colour: String,
    pub subtitle_position: u32,
    pub subtitle_font: VideoPlayerSubtitleFont,

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
            on_left_click: VideoPlayerOnLeftClick::default(),
            duration_display: DurationDisplay::default(),

            subtitle_scale: 1.0,
            subtitle_colour: "#FFFFFFFF".into(),
            subtitle_background_colour: "#00000000".into(),
            subtitle_position: 100,
            subtitle_font: VideoPlayerSubtitleFont::default(),

            backend: VideoPlayerBackendPreference::default(),
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

impl From<VideoPlayerSkipAmount> for Duration {
    fn from(value: VideoPlayerSkipAmount) -> Self {
        Duration::from_secs(value as u64)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum VideoPlayerOnLeftClick {
    #[default]
    PlayPause,
    ToggleControls,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum VideoPlayerBackendPreference {
    #[default]
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

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum DurationDisplay {
    #[default]
    Total,
    Remaining,
}

impl DurationDisplay {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Total => Self::Remaining,
            Self::Remaining => Self::Total,
        }
    }
}
