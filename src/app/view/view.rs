#![allow(unused_imports, unused_braces)]
use iced::{Element};
use iced::widget::{button, text, Column, Row, Button, progress_bar};
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
            .push(progress_bar(0.0..=self.song_duration(), self.current_pos))
            .push(button("Song duration").on_press(Audio::Duration))
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
