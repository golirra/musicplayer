#![allow(dead_code, unused_imports, unused_results)]
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
use crate::app::state::db::scanner::Metadata;

use rusqlite::Connection;


use iced::time;
use iced::Length;
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
    pub volume: f32,
    pub song_length: Option<Duration>, 
    pub current_pos: f32,
    pub playback_sink: Option<Sink>,
    _audio_stream: Option<OutputStream>,
    files: Vec<Arc<(String, Metadata)>>,
    current_song_index: usize,
    pub image: String,
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
            current_song_index: 0,
            image: "
            C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/cat.jpg".to_string(),
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
                println!("files len:{}", self.files.len());
                // self.current_song_index = self.current_song_index % self.files.len();
                // println!("song index:{}", self.current_song_index);
                Task::none()
            },
            Audio::Prev => {
                dbg!("{}",&self.current_song_index);
                // self.prev_song();
                Task::none()
            },
            Audio::Next => {
                // self.next_song();
                Task::none()
            },
            Audio::Volume(volume) => {
                self.volume(volume);
                Task::none()
            }
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
                self.files = scanner::get_paths_with_metadata().unwrap().into_iter().map(Arc::new).collect();
                Task::none()
            },
            _ => {Task::none()},
        }
    }

    pub fn subscription(&self) -> Subscription<Audio> {
        // println!("{}",self.current_song_index);
        time::every(Duration::from_millis(500)).map(|_instant| Audio::PlaybackTick)
        // Update every second
    }

    pub fn load_audio(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        let file = fs::File::open(file_path)?;
        // let file2 = fs::File::open("C:/Users/webbs/programming/cs/rust/Rust-playground/src/Music/Shot_Forth_Self_Living/03.Medicine-Defective")?;
        // let decoder2 = rodio::Decoder::try_from(file2)?;


        let decoder = rodio::Decoder::try_from(file)?;
        match decoder.total_duration() {
            Some(duration) => self.song_length = Some(duration),
            None => eprintln!("Warning: unable to determine song duration."),
        }

        sink.append(decoder);
        sink.set_volume(self.volume);

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

    /*
    fn prev_song(&mut self) { 
        // Check if we are at the first song, then wrap around to the last one
        self.current_song_index = if self.current_song_index == 0 {
            self.files.len() - 1  // Go to the last song
        } else {
            self.current_song_index - 1  // Go to the previous song
        };

        let prev_song = self.files[self.current_song_index].clone();
        self.load_audio(&prev_song);
    }

    fn next_song(&mut self) {
        self.current_song_index = (self.current_song_index + 1) % self.files.len();
        let next_song = self.files[self.current_song_index].clone();
        self.load_audio(&next_song);
    }
    */

    fn volume (&mut self, vol: f32) {
        if let Some(sink) = &self.playback_sink {
            self.playback_sink.as_mut().unwrap().set_volume(vol);
        } else {
            self.volume = vol;
            println!("shidddd :DDD");
        }
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
            .fold(Column::new(), |column, tuple| {
                column.push(
                    button(tuple.1.title.as_str())
                    .on_press(Audio::Play(tuple.0.clone()))
                    .height(Length::Shrink)
                    .width(Length::Shrink))
            })
    }
}

