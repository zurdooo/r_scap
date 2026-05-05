mod audio;
mod video;

pub use audio::*;
pub use video::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Frame {
    Audio(AudioFrame),
    Video(VideoFrame),
}
