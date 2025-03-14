#![allow(dead_code)]
//Represents the actions taken when a button is pressed
use std::sync::Arc;
#[derive(Clone, Debug)]
pub enum Audio {
    Load,
    Play(Arc<String>),
    Stop,
    TogglePlayPause,
    Pause,
    Previous,
    Next,
    RandomNextTrack,
    Duration,
    SliderPositionChanged(f32),
    UpdatePlaybackPosition(f32),
    PlaybackTick,
    Test,
    ShowFiles,
}

pub enum File {
    Load,
    Display,
    Duration,
}
