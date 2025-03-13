use iced::{Element};
use iced::widget::{button, Column, Row, Button};
use crate::app::state::audio::AudioState; // Reference the state controller
use crate::app::message::Audio;
use crate::app::view::playlist;

//static location for playback bar button definitions, did this so I could iterate over them
pub const BUTTONS: &[(&'static str, Audio); 6] = &[
    ("Load audio", Audio::Load),
    ("Stop", Audio::Stop),
    ("Play/Pause", Audio::TogglePlayPause),
    ("Pause", Audio::Pause),
    ("Previous", Audio::Previous),
    ("Next", Audio::Next),
];

impl AudioState {

    pub fn view(&self) -> Element<Audio> {
        Column::new()
            .push(Self::playback_controls())
            .into()

    }


    pub fn playback_controls() -> Row<'static, Audio> { 
        let playback_controls = BUTTONS
            .iter()
            .fold(Row::new(), |row, (label, action)| {
                row.push(button(*label).on_press(action.clone()))
            });
        playback_controls.into()
    }

}
