#[allow(dead_code, unused_imports)]
use iced::widget::{button, Button, column, container, progress_bar, row, slider, text, Text, Column};
use iced::{Element, Theme};

#[derive(Debug, Clone)]
enum PlaybackComponent {
    PlaybackButton,
    PlaybackBar,
    ProgressBar,
    VolumeBar,
}

pub struct Layout {

}
