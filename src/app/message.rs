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
    Prev,
    Next,
    RandomNextTrack,
    Duration,
    SliderPositionChanged(f32),
    UpdatePlaybackPosition(f32),
    PlaybackTick,
    Test,
    ShowFiles,
}

#[derive(Clone, Debug)]
pub enum File {
    Load,
    Display,
    Duration,
}

#[derive(Clone, Debug)]
pub enum Draggable {
    Press,
    Release,
}

#[derive(Clone, Debug)]
pub enum Message {
    Audio(Audio),
    File(File),
}

