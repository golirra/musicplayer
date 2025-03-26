#![allow(unused_imports, unused_braces, dead_code)]
mod app;
use std::env;
use std::path::Path;
use iced::{Theme, Element, Task, Subscription};
use iced::widget::{button, Button, Column};
use anyhow::Result;
// use iced::widget::image::{Image, Handle};

//TODO: change re-export names in all modules for readability
use crate::app::view::playlist;
use crate::app::state::audio::AudioState; 
use crate::app::message::{Audio, File};
use crate::app::message::Message;
use crate::app::state::playlist::FileState;
use crate::app::state::db::scanner;
fn main() -> iced::Result {
    scanner::setup_database();
    // dbg!(scanner::get_paths_with_metadata());
    iced::application(
        "Test application",
        Controller::update,
        Controller::view,
    )
    .subscription(Controller::subscription)
    .resizable(true)
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

    //NOTE:since self.audio.view() returns an Element<Audio>,iced lets us map over
    //self.audio.view() to wrap the Audio element in a Message::Audio variant
    pub fn view(&self) -> Element<Message>  {
        let w = FileState::new();
        let v = self.audio.view().map(|audio_msg| Message::Audio(audio_msg));
        let x = self.files.view().map(|file_msg| Message::File(file_msg));
        let y = Column::new()
            .push(v)
            .push(x);
        y.into()
        
        // let x = Handle::from_path("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/cat.jpg");
        // let y: Image<Handle> = Image::new(x);
        // y.opacity(1.0).into()
        // v.into()
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
                Task::none()
            },
            Message::Audio(audio_msg) => {
                let _ = self.audio.update(audio_msg);
                Task::none()
            },
            Message::File(File::Select(path)) => {
                println!("file selected: {}", path);
                self.audio.update(Audio::Play(path));
                Task::none()
            },
            Message::File(file_msg) => {
                let _ = self.files.update(file_msg);
                Task::none()
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // self.audio.subscription()
        self.audio.subscription().map(|audio_msg| Message::Audio(audio_msg))
    }
}

