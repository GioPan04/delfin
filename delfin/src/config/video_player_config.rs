use std::{cell::RefCell, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::video_player::backends::{
    gst::VideoPlayerBackendGst, mpv::VideoPlayerBackendMpv, VideoPlayerBackend,
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
            VideoPlayerBackendPreference::Gst => Arc::<RefCell<VideoPlayerBackendGst>>::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct VideoPlayerConfig {
    pub position_update_frequency: usize,
    pub skip_backwards_amount: VideoPlayerSkipAmount,
    pub skip_forwards_amount: VideoPlayerSkipAmount,
    pub backend: VideoPlayerBackendPreference,
    pub hls_playback: bool,
    pub intro_skipper: bool,
}

impl Default for VideoPlayerConfig {
    fn default() -> Self {
        Self {
            position_update_frequency: 10,
            skip_backwards_amount: VideoPlayerSkipAmount::Ten,
            skip_forwards_amount: VideoPlayerSkipAmount::Thirty,
            backend: VideoPlayerBackendPreference::Mpv,
            hls_playback: false,
            intro_skipper: true,
        }
    }
}

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum VideoPlayerSkipAmount {
    Ten = 10,
    Thirty = 30,
}
