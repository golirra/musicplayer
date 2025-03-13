#![allow(dead_code, unused_imports)]
use std::sync::Arc;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::BufReader;
use std::time::Duration;
use std::vec;
use tokio;
use crate::Audio;


use iced::time;
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::widget::{Button, Column, Container};
use iced::{Subscription, Renderer, Theme, Element, Task, Fill};

use rodio::{Decoder, OutputStream, Sink};

#[derive(Default)]
pub struct AudioState {
    volume: f32,
    current_position: f32,
    playback_sink: Option<Sink>,
    _audio_stream: Option<OutputStream>,
    pub files: Vec<Arc<String>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            volume: 0.0,
            current_position: 0.0,
            playback_sink: None,
            _audio_stream: None,
            files: vec![],
        }
    }

    pub fn update(&mut self, message: Audio) -> Task<Audio> {
        match message {
            Audio::Load => {
                self.load_audio("C:/Users/webbs/programming/cs/rust/musicplayer/src/app/state/song.mp3");
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
    pub fn load_audio(&mut self, file_path: &str) {
        if self.playback_sink.is_none() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let file = BufReader::new(fs::File::open(file_path).unwrap());
            let source = Decoder::new(file).unwrap();

            sink.append(source);
            sink.set_volume(0.2);

            self._audio_stream = Some(stream);
            self.playback_sink = Some(sink);
        }
    }

    pub fn update_playback_position(&mut self) {
        if let Some(sink) = &self.playback_sink {
            self.current_position = sink.get_pos().as_secs_f32();
        }
    }

    pub fn song_progress(&self) -> u64 {
        match &self.playback_sink {
            Some(sink) => sink.get_pos().as_secs(),
            None => 0,
        }
    }

    pub fn get_filenames_in_directory() -> Vec<String> {
        fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect()
    }
}

