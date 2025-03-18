#![allow(dead_code, unused_imports)]
use std::sync::Arc;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufReader, Error as ioError};
use std::time::Duration;
use std::vec;
use tokio;
use crate::Audio;
use crate::Message;
use crate::app::view::playlist;
use crate::app::state::db::scanner;

use rusqlite::Connection;


use iced::time;
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::widget::{Button, Column, Container};
use iced::{Subscription, Renderer, Theme, Element, Task, Fill};

use rodio::{Decoder, OutputStream, Sink, Source};
const TEST_SONG: &str = "C:/Users/webbs/programming/cs/rust/musicplayer/src/app/state/song.mp3";

#[derive(Debug)]
pub enum AudioError {
    NoAudioPlaying,
    PlayError(rodio::PlayError),
}

impl From<rodio::PlayError> for AudioError {
    fn from(err: rodio::PlayError) -> Self {
        AudioError::PlayError(err)
    }
}

#[derive(Default)]
pub struct AudioState {
    volume: f32,
    pub song_length: Option<Duration>, 
    pub current_pos: f32,
    playback_sink: Option<Sink>,
    _audio_stream: Option<OutputStream>,
    pub files: Vec<Arc<String>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            volume: 0.0,
            song_length: None,
            current_pos: 0.0,
            playback_sink: None,
            _audio_stream: None,
            files: vec![],
        }
    }

    pub fn update(&mut self, message: Audio) -> Task<Audio> {
        match message {
            Audio::Load => {
                //NOTE: use let _ to ignore the resulting value. not sure what the point is though
                let _ = self.load_audio(TEST_SONG);
                Task::none()
            },
            Audio::Stop => {
                self.stop_audio();
                self.current_pos = 0.0; 
                Task::none()
            },
            Audio::Play(file) => {
                self.load_audio(&file);
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
            //TODO:
            Audio::ShowFiles => {
                // self.files = playlist::Playlist::get_filenames_in_directory().into_iter().map(Arc::new).collect();
                self.files = scanner::read_table().unwrap().into_iter().map(Arc::new).collect();
                Task::none()
            },
            _ => {Task::none()},
        }
    }

    pub fn subscription(&self) -> Subscription<Audio> {
        println!("x");
        time::every(Duration::from_millis(500)).map(|_instant| Audio::PlaybackTick)
        // Update every second
    }

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
    fn stop_audio(&mut self) -> Result<(), AudioError>   {
        if let Some(sink) = &self.playback_sink {
            sink.stop();
            Ok(())
        } else {
            Err(AudioError::NoAudioPlaying)
        }
    }
    // Function to load audio using the song name from the database
    //TODO: 
    pub fn load_audio_from_db(&mut self, conn: &Connection, song_name: &str) -> Result<(), Box<dyn Error>> {
        todo!();
        // Get the file path from the database
        // let song_path = database::get_song_path(conn, song_name)?;

        // Use the load_audio function to load the song
        // self.load_audio(&song_path)
    }
    pub fn update_playback_position(&mut self) {
        if let Some(sink) = &self.playback_sink {
            self.current_pos = sink.get_pos().as_secs_f32();
        } else {
            self.current_pos = 0.0;
        }
        
    }

    pub fn song_duration(&self) -> f32 {
        self.song_length.map(|d| d.as_secs_f32())
            .unwrap_or(0.0)
    }

    pub fn get_filenames_in_directory() -> Vec<String> {
        fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect()
    }
    //TODO: Format button names properly
    pub fn files_as_buttons(&self) -> Column<Audio> {
         self.files
            .iter()
            .fold(Column::new(), |column, filename| {
                column.push(button(filename.as_str()).on_press(Audio::Play(filename.clone())))
            })
               
    }
}

