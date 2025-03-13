mod app;
use iced::Theme;

<<<<<<< Updated upstream
use crate::app::state::state::AudioState;
use crate::app::message::Audio;
=======

use crate::app::state::audio::AudioState; //TODO: change re-exports in app module for readability
use crate::app::state::files::FileState;
use crate::app::message::ControllerMessage;
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
=======
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

    pub fn view(&self) -> Element<ControllerMessage> {
        self.audio.view().map(ControllerMessage::Audio)
    }
    
    pub fn update(&mut self, message: ControllerMessage) -> Task<ControllerMessage> {
        match message {
            ControllerMessage::Test => {
                println!("Test works");
                Task::none()
            },
            ControllerMessage::Audio(audio_msg) => {
                self.audio.update(audio_msg).map(ControllerMessage::Audio)
            },
            ControllerMessage::File(file_msg) => {
                self.files.update(file_msg).map(ControllerMessage::File)
            },
        }
    }
}
>>>>>>> Stashed changes

