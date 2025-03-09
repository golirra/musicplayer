mod button;
mod playback;
use crate::button::PlaybackController;
use iced::Subscription;

fn main() -> iced::Result {
   
    //iced::run("Play me some music", playback::update, playback::view)
    //iced::run("Play me some music", PlaybackController::update, PlaybackController::view)
    iced::application("Music", PlaybackController::update, PlaybackController::view)
        .subscription(PlaybackController::subscription)
        .run()

}
