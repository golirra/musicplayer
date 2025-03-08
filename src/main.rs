mod button;
use crate::button::PlaybackController;
fn main() -> iced::Result {
   
    iced::run("Play me some music", PlaybackController::update, PlaybackController::view)
}
