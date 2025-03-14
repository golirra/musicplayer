#![allow(unused_imports, unused_braces, dead_code)]
mod app;
use iced::{Theme, Element, Task, Subscription};
use iced::widget::{button, Column};


use crate::app::state::audio::AudioState; //TODO: change re-exports in app module for readability
use crate::app::message::Audio;
use crate::app::message::Message;
use crate::app::state::files::FileState;
fn main() -> iced::Result {

    iced::application(
        "Test application",
        Controller::update,
        Controller::view,
    )
    .subscription(Controller::subscription)
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

    //NOTE:since self.audio.view() returns an Element<Audio> we can map over
    //self.audio.view() to wrap the Audio element in a Message::Audio variant
    pub fn view(&self) -> Element<Message> {
        self.audio.view().map(|audio_msg| Message::Audio(audio_msg))
    }
    
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Audio(Audio::Test) => {
                println!("Test works");
                Task::none()
            },
            Message::Audio(Audio::PlaybackTick) => {
                self.audio.update_playback_position();
                Task::none()
            },
            Message::Audio(Audio::Duration) => {
                dbg!(self.audio.song_duration());
                dbg!("in main::Duration");
                Task::none()
            },
            Message::Audio(audio_msg) => {
                self.audio.update(audio_msg);
                Task::none()
            },
            Message::File(_) => {
                Task::none()
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // self.audio.subscription()
        self.audio.subscription().map(|audio_msg| Message::Audio(audio_msg))
    }


}

