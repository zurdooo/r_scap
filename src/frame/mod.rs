mod audio;
mod video;

pub use audio::*;
pub use video::*;

use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum Frame {
    Audio(AudioFrame),
    Video(VideoFrame),
}
