mod app;
use iced::{Theme, Element, Task};
use iced::widget::{button, Column};


use crate::app::state::audio::AudioState; //TODO: change re-exports in app module for readability
use crate::app::message::Audio;
use crate::app::state::files::FileState;
fn main() -> iced::Result {

    iced::application(
        "Test application",
        Controller::update,
        Controller::view,
    )
    //.subscription(AudioState::subscription)
    .resizable(false)
    .theme(|_| Theme::Light)
    .run()

}
//NOTE:To delegate control to other areas of the program, create a struct whose fields are the "sub
//controllers" and then call the relevant function in update() or view()
//example: self.audio.update(message) gets called in update()
#[derive(Default)]
pub struct Controller {
    audio: AudioState,
    files: FileState,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            audio: AudioState::new(),
            files: FileState::new(),
        }
    }

    pub fn view(&self) -> Element<Audio> {
        self.audio.view()
    }
    
    pub fn update(&mut self, message: Audio) -> Task<Audio> {
        match message {
            Audio::Test => {
                println!("Test works");
                Task::none()
            },
            _ => {
                self.audio.update(message)
            },
        }
    }


}

