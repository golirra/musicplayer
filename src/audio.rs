//Anything that interacts with rodio goes in this file.

#![allow(dead_code, unused_imports)]
use std::sync::Arc;
use std::fs;
use std::io::BufReader;
use std::time::Duration;
use std::vec;
use tokio;

use crate::file;
use crate::utils;
use crate::layout;

use iced::time;
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::widget::{Button, Column, Container};
use iced::{Subscription, Renderer, Theme, Element, Task, Fill};

use rodio::{Decoder, OutputStream, Sink};
//Represents the actions taken when a button is pressed
#[derive(Clone, Debug)]
pub enum Audio {
    Load,
    Play(Arc<String>),
    Stop,
    TogglePlayPause,
    Pause,
    Previous,
    Next,
    RandomNextTrack,
    SliderPositionChanged(f32),
    UpdatePlaybackPosition(f32),
    PlaybackTick,
    Test,
    ShowFiles,
}

#[derive(Default)]
pub struct AudioPlaybackController {
    volume: f32,
    current_position: f32,
    playback_sink: Option<Sink>,
    _audio_stream: Option<OutputStream>,
    files: Vec<Arc<String>>,//might need to change in future based on how expensive Arc is
}
//TODO: Move all the UI stuff (like BUTTONS etc) to a new file, only audio functionality like
//loading a song should be here
impl AudioPlaybackController {
    pub fn new() -> Self {
        Self {
            volume: 0.0,
            current_position: 0.0,
            playback_sink: None,
            _audio_stream: None,
            files: vec![],
        }
    }

    pub fn view(&self) -> Element<Audio> {
        //label, action are defined in BUTTONS array
        let files = Self::get_filenames_in_directory();
        self.files_as_buttons()
            .push(button("load").on_press(Audio::ShowFiles)).into()
            
    }

    pub fn update(&mut self, message: Audio) -> Task<Audio> {
        match message {
            Audio::Load => {
                self.load_audio();
                Task::none()
            },
            //TODO: Make Play work !
            Audio::Play(filename) => {
                println!("Playing: {}", filename);
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
            Audio::SliderPositionChanged(value) => {
                self.volume = value;
                Task::none()
            },
            Audio::PlaybackTick => {
                self.update_playback_position();
                Task::none()
            },
            //map(|filename| Arc::new(filename)) is just a more verbose way of map(Arc::new) to
            //help me remember that its technically a closure
            Audio::ShowFiles => {
                self.files = Self::get_filenames_in_directory().into_iter().map(|filename| Arc::new(filename)).collect();
                Task::none()
            },
            _ => {
                Task::none()
            }
        }
    }

    //TODO: Only track time when source is playing
    pub fn subscription(&self) -> Subscription<Audio> {
        println!("x");
        time::every(Duration::from_secs(1)).map(|_instant| Audio::PlaybackTick)
        // Update every second
    }

    pub fn load_audio(&mut self) {
        if self.playback_sink.is_none() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let file = BufReader::new(fs::File::open("song.mp3").unwrap());
            let source = Decoder::new(file).unwrap();

            sink.append(source);
            sink.set_volume(0.2);

            self._audio_stream = Some(stream);
            self.playback_sink = Some(sink);
        }
    }

    pub fn update_playback_position(&mut self) {
        if let Some(sink) = &self.playback_sink {
            let _ = self.update(Audio::SliderPositionChanged(
                sink.get_pos().as_secs_f32(),
            ));
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(sink) = &self.playback_sink {
            sink.play();
            self.volume = 51.0;
        }
    }

    pub fn pause_audio(&mut self) {
        if let Some(sink) = &self.playback_sink {
            sink.pause();
        }
    }

    pub fn get_filenames_in_directory() -> Vec<String> {
        fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect()
    }

    //.push(progress_bar(0.0..=100.0, self.volume))
    pub fn song_progress(&self) -> u64 {
        
        match &self.playback_sink {
            Some(sink) => sink.get_pos(),
            None => Duration::new(5, 0),
        }
            .as_secs()

    }

    pub fn files_as_buttons(&self) -> Column<Audio> {
        let files = Column::new()
            .push(
                self.files
                    .iter()
                    .fold(Column::new(), |column, filename| {
                        column.push(button(filename.as_str()).on_press(Audio::Play(filename.clone())))
                    }),
            );
        files
    }
}
