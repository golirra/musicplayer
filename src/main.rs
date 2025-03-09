mod button;
mod file;

use crate::button::AudioPlaybackController;

fn main() -> iced::Result {
   
    //iced::run("Play me some music", playback::update, playback::view)
    //iced::run("Play me some music", AudioPlaybackController::update, PlaybackController::view)
    iced::application("Music", AudioPlaybackController::update, AudioPlaybackController::view)
        .subscription(AudioPlaybackController::subscription)
        .run()

}
