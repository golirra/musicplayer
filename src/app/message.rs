//Represents the actions taken when a button is pressed
use std::sync::Arc;

//NOTE: the syntax Audio(Audio) allows us to place data directly inside the enum variant.
#[derive(Clone, Debug)]
pub enum ControllerMessage {
    Audio(Audio), 
    File(File),
    Test,
}

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
<<<<<<< Updated upstream
=======

#[derive(Clone, Debug)]
pub enum File {
    Load,
    Display,
}

>>>>>>> Stashed changes
