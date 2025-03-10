mod audio;
mod file;
mod layout;
use iced::Theme;

use crate::audio::AudioPlaybackController;

fn main() -> iced::Result {
    println!("Test");

    //iced::run("Play me some music", AudioPlaybackController::update, PlaybackController::view)
    iced::application("Test application", AudioPlaybackController::update, AudioPlaybackController::view)
        .subscription(AudioPlaybackController::subscription)
        .resizable(false)
        .theme(|_| Theme::Light)
        .run()

    /*
    iced::application("Music", AudioPlaybackController::update, AudioPlaybackController::view)
        .subscription(AudioPlaybackController::subscription)
        .run()
    */
}
