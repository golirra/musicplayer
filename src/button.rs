#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;

use iced::{ Element };
use iced::widget::{button, text, column};
use iced::window;

use rodio::{Decoder, OutputStream, Sink};

#[derive(Clone, Debug)]
pub enum PlaybackButton {
    Stop,
    Play,
    Pause,
    Previous,
    Next,
    NextRand,
}

#[derive(Default)]
pub struct ButtonApp {
    value: u64,
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
}

#[derive(Clone, Debug)]
enum Message {
    ButtonPressed,
}

impl ButtonApp {

    pub fn new() -> Self {
        Self {
            value: 0,
            sink: None,
            _stream: None,
            //button_state: button::State::new(),
        }
    }

    pub fn view(&self) -> Element<PlaybackButton> {
        column![
            button("+").on_press(PlaybackButton::Play),
            text(self.value).size(200),
        ]
            .into()
    }

    pub fn update(&mut self, message: PlaybackButton) {
        match message {
            PlaybackButton::Play => {
                self.value += 1;
                self.play_audio();
            },
            _ => (),
        }
    }

    pub fn play_audio(&mut self) {
        if self.sink.is_none() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let file = BufReader::new(File::open("song.mp3").unwrap());
            let source = Decoder::new(file).unwrap();

            sink.append(source);
            sink.play();

            self._stream = Some(stream);
            self.sink = Some(sink);
            std::thread::sleep(std::time::Duration::from_secs(1));
        } else if let Some(sink) = &self.sink {
            sink.stop();
            sink.play();
        }
    }
    
}
