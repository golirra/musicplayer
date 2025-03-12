//Manages the overall layout of the UI
#[allow(dead_code, unused_imports)]
use crate::audio;

use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::widget::{Button, Column, Row, Text};
use iced::{Element, Theme};

pub const BUTTONS: &[(&'static str, audio::Audio); 6] = &[
    ("Load audio", audio::Audio::Load),
    ("Stop", audio::Audio::Stop),
    ("Play/Pause", audio::Audio::TogglePlayPause),
    ("Pause", audio::Audio::Pause),
    ("Previous", audio::Audio::Previous),
    ("Next", audio::Audio::Next),
];

#[derive(Debug, Clone)]
enum PlaybackComponent {
    PlaybackButton,
    PlaybackBar,
    ProgressBar,
    VolumeBar,
}


pub fn playback_controls() -> Row<'static, audio::Audio> { 
    let playback_controls = BUTTONS
        .iter()
        .fold(Row::new(), |row, (label, action)| {
            row.push(button(*label).on_press(action.clone()))
        });
    playback_controls.into()
}

//Create a const array of buttons to iterate over instead of making buttons by spamming ".push(button)"
//Also worth noting that you can't use "let" out here because "let" is a runtime variable and this
//stuff has to be known before the program is running
