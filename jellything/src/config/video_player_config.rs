use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct VideoPlayerConfig {
    pub position_update_frequency: usize,
    pub skip_backwards_amount: VideoPlayerSkipAmount,
    pub skip_forwards_amount: VideoPlayerSkipAmount,
    pub hls_playback: bool,
}

impl Default for VideoPlayerConfig {
    fn default() -> Self {
        Self {
            position_update_frequency: 10,
            skip_backwards_amount: VideoPlayerSkipAmount::Ten,
            skip_forwards_amount: VideoPlayerSkipAmount::Thirty,
            hls_playback: true,
        }
    }
}

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum VideoPlayerSkipAmount {
    Ten = 10,
    Thirty = 30,
}
