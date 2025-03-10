#[allow(dead_code, unused_imports)]
use iced::widget::{
    button, column, container, progress_bar, row, slider, text, Button, Column, Text,
};
use iced::{Element, Theme};

#[derive(Debug, Clone)]
enum PlaybackComponent {
    PlaybackButton,
    PlaybackBar,
    ProgressBar,
    VolumeBar,
}

pub struct Layout {}
