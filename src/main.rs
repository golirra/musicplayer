mod audio;
mod test;

mod file;
mod layout;
mod utils;
use iced::Theme;

use crate::audio::AudioPlaybackController;

fn main() -> iced::Result {
    println!("Test");

    /*
    iced::application(
        "Test application",
        AudioPlaybackController::update,
        AudioPlaybackController::view,
    )
    .subscription(AudioPlaybackController::subscription)
    .resizable(false)
    .theme(|_| Theme::Light)
    .run()
    */


    iced::application(
        "Test application",
        test::update,
        test::view,
    )
    .run()

}
