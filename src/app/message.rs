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
    SliderPositionChanged(f32),
    UpdatePlaybackPosition(f32),
    PlaybackTick,
    Test,
    ShowFiles,
}
