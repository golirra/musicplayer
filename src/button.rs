#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;

use iced::{Fill, Element };
use iced::widget::{button, column, container, row, text};
use iced::window;

use rodio::{Decoder, OutputStream, Sink};


#[derive(Clone, Debug)]
pub enum PlaybackAction {
    Load,
    Stop,
    Play,
    Pause,
    Previous,
    Next,
    NextRand,
}

#[derive(Default)]
pub struct PlaybackController {
    value: u64,
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
}

#[derive(Clone, Debug)]
enum Message {
    ButtonPressed,
}

impl PlaybackController {

    pub fn new() -> Self {
        Self {
            value: 0,
            sink: None ,
            _stream: None,
            //button_state: button::State::new(),
        }
    }

    pub fn view(&self) -> Element<PlaybackAction> {
        container(
            row![
                button("Load audio").on_press(PlaybackAction::Load),
                button("Stop").on_press(PlaybackAction::Stop),
                button("Play").on_press(PlaybackAction::Play),
                button("Pause").on_press(PlaybackAction::Pause),
                button("Previous").on_press(PlaybackAction::Previous),
                button("Next").on_press(PlaybackAction::Next),
                button("NextRand").on_press(PlaybackAction::NextRand),
                text(self.value).size(20),
            ]
            .spacing(1)
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn update(&mut self, message: PlaybackAction) {
        match message {
            PlaybackAction::Load => {
                self.load_audio();
            },
            PlaybackAction::Play => {
                self.play_audio();
            },
            PlaybackAction::Pause => {
                self.pause_audio();
            },
                
            _ => {}, 
        }
    }

    pub fn load_audio(&mut self) {
        if self.sink.is_none() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let file = BufReader::new(File::open("song.mp3").unwrap());
            let source = Decoder::new(file).unwrap();

            sink.append(source);

            self._stream = Some(stream);
            self.sink = Some(sink);
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn pause_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_playback() {
        let mut controller = PlaybackController::new();
        controller.update(PlaybackAction::Play);
    }
}
