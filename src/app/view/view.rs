use iced::{Element};
use iced::widget::{button, Column, Row, Button};
use crate::app::state::state::AudioState; // Reference the state controller
use crate::app::message::Audio;

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

        Self::playback_controls().into()

    }

    pub fn files_as_buttons(&self) -> Column<Audio> {
        self.files.iter().fold(Column::new(), |column, filename| {
            column.push(button(filename.as_str()).on_press(Audio::Play(filename.clone())))
        })
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
