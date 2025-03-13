mod app;
use iced::Theme;

use crate::app::state::state::AudioState;
use crate::app::message::Audio;
fn main() -> iced::Result {

    iced::application(
        "Test application",
        AudioState::update,
        AudioState::view,
    )
    .subscription(AudioState::subscription)
    .resizable(false)
    .theme(|_| Theme::Light)
    .run()

}

