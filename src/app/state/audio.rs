#![allow(dead_code, unused_imports)]
use std::sync::Arc;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::BufReader;
use std::time::Duration;
use std::vec;
use tokio;
use crate::Audio;
use crate::app::view::playlist;
use crate::Message;


use iced::time;
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::widget::{Button, Column, Container};
use iced::{Subscription, Renderer, Theme, Element, Task, Fill};

use rodio::{Decoder, OutputStream, Sink, Source};

#[derive(Default)]
pub struct AudioState {
    volume: f32,
    pub song_length: Option<Duration>, 
    pub current_pos: f32,
    playback_sink: Option<Sink>,
    _audio_stream: Option<OutputStream>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            volume: 0.0,
            song_length: None,
            current_pos: 0.0,
            playback_sink: None,
            _audio_stream: None,
        }
    }

    pub fn update(&mut self, message: Audio) -> Task<Audio> {
        match message {
            Audio::Load => {
                //NOTE: use let _ to ignore the resulting value. not sure what the point is though
                let _ = self.load_audio("C:/Users/webbs/programming/cs/rust/musicplayer/src/app/state/song.mp3");
                Task::none()
            },
            Audio::TogglePlayPause => {
                if let Some(sink) = &self.playback_sink {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                Task::none()
            },
            //NOTE: a playback tick is defined by the subscription function which passively listens
            // to an event and does something
            Audio::PlaybackTick => {
                self.update_playback_position();
                Task::none()
            },
            Audio::Duration => {
                dbg!("{}", self.song_length);
                Task::none()
            },


            _ => {Task::none()},
        }
    }

    //TODO: Only track time when source is playing
    pub fn subscription(&self) -> Subscription<Audio> {
        println!("x");
        time::every(Duration::from_secs(1)).map(|_instant| Audio::PlaybackTick)
        // Update every second
    }

    //TODO: Make file_path not hardcoded 
    pub fn load_audio(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        let file = fs::File::open(file_path)?;
        let decoder = rodio::Decoder::try_from(file)?;
        match decoder.total_duration() {
            Some(duration) => self.song_length = Some(duration),
            None => eprintln!("Warning: unable to determine song duration."),
        }

        sink.append(decoder);
        sink.set_volume(0.2);

        self._audio_stream = Some(stream_handle);
        self.playback_sink = Some(sink);

        Ok(())
    }

    pub fn update_playback_position(&mut self) {
        if let Some(sink) = &self.playback_sink {
            self.current_pos = sink.get_pos().as_secs_f32();
        }
    }

    pub fn song_duration(&self) -> f32 {
        self.song_length.map(|d| d.as_secs_f32())
            .unwrap_or(0.0)
    }

    pub fn song_progress(&self) -> u64 {
        match &self.playback_sink {
            Some(sink) => sink.get_pos().as_secs(),
            None => 0,
        }
    }

}

