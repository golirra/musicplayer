#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;


use iced::{executor, time, Application };
use iced::Subscription;
use iced::{Fill, Element };
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::window;
use crate::playback;

use rodio::{Decoder, OutputStream, Sink, Source};

#[derive(Clone, Debug)]
pub enum PlaybackAction {
    Load,
    Stop,
    Play,
    Pause,
    Previous,
    Next,
    NextRand,
    SliderChanged(f32),
    UpdatePos(f32),
    Tick,
}

pub struct State {
    value: f32,
}

#[derive(Default)]
pub struct PlaybackController {
    value: f32,
    song_pos: f32,
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
            value: 0.0,
            song_pos: 0.0,
            sink: None ,
            _stream: None,
            //button_state: button::State::new(),
        }
    }
    
    pub fn view(&self) -> Element<PlaybackAction> {
        container(
            row![
            slider(0.0..=100.0, self.value, PlaybackAction::SliderChanged),
            progress_bar(0.0..=100.0, self.value), 
            text(
                match &self.sink {
                    Some(x) => x.get_pos(),
                    None => Duration::new(5, 0),
                }.as_secs()
            ),
            container(
                row![
                button("Load audio").on_press(PlaybackAction::Load),
                button("Stop").on_press(PlaybackAction::Stop),
                button("Play").on_press(PlaybackAction::Play),
                button("Pause").on_press(PlaybackAction::Pause),
                button("Previous").on_press(PlaybackAction::Previous),
                button("Next").on_press(PlaybackAction::Next),
                button("NextRand").on_press(PlaybackAction::NextRand),
                ]
                .spacing(1)
            )
            ]
        )
            .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<PlaybackAction> {
        println!("Subscription started");
        time::every(Duration::from_secs(1)).map(|_instant| PlaybackAction::Tick) // Update every second
    }

    pub fn update(&mut self, message: PlaybackAction) {
        match message {
            PlaybackAction::Load => {
                self.load_audio();
            },
            PlaybackAction::Play => {
                if let Some(sink) = &self.sink {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
            },
            PlaybackAction::SliderChanged(value) => {
                self.value = value;
            },
            PlaybackAction::UpdatePos(value) => {
            },
            PlaybackAction::Tick => {
                self.update_song_pos();
                println!("Tick");
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
            

            /*
            source.periodic_access(Duration::from_secs(1), |source|{
                self.update_song_pos();
            });
            */
            sink.append(source);
            sink.set_volume(0.05);
            

            self._stream = Some(stream);
            self.sink = Some(sink);
            
        }
    }

    pub fn update_song_pos(&mut self) {
        if let Some(sink) = &self.sink {
            self.update(PlaybackAction::SliderChanged(sink.get_pos().as_secs_f32()));
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.value = 51.0;
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
