//Anything that interacts with rodio goes in this file.

#![allow(dead_code, unused_imports)]
use std::fs;
use std::io::BufReader;
use std::time::Duration;
use std::vec;
use tokio;

use crate::file;
use crate::utils;

use iced::time;
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::widget::{Column, Container};
use iced::{Subscription, Theme, Element, Task, Fill};

use rodio::{Decoder, OutputStream, Sink};
//Represents the actions taken when a button is pressed
#[derive(Copy, Clone, Debug)]
pub enum AudioAction {
    LoadAudio,
    StopPlayback,
    TogglePlayPause,
    PausePlayback,
    PreviousTrack,
    NextTrack,
    RandomNextTrack,
    SliderPositionChanged(f32),
    UpdatePlaybackPosition(f32),
    PlaybackTick,
    Test,
}

#[derive(Default)]
pub struct AudioPlaybackController {
    volume: f32,
    current_position: f32,
    playback_sink: Option<Sink>,
    _audio_stream: Option<OutputStream>,
}

//Create a const array of buttons to iterate over instead of making buttons by spamming ".push(button)"
//Also worth noting that you can't use "let" out here because "let" is a runtime variable and this
//stuff has to be known before the program is running
const BUTTONS: [(&str, AudioAction); 6] = [
    ("Load audio", AudioAction::LoadAudio),
    ("Stop", AudioAction::StopPlayback),
    ("Play/Pause", AudioAction::TogglePlayPause),
    ("Pause", AudioAction::PausePlayback),
    ("Previous", AudioAction::PreviousTrack),
    ("Next", AudioAction::NextTrack),
];

//TODO: Move all the UI stuff (like BUTTONS etc) to a new file, only audio functionality like
//loading a song should be here
impl AudioPlaybackController {
    pub fn new() -> Self {
        Self {
            volume: 0.0,
            current_position: 0.0,
            playback_sink: None,
            _audio_stream: None,
        }
    }

    pub fn view(&self) -> Element<AudioAction> {
        //label, action are defined in BUTTONS array
        let files = Self::get_filenames_in_directory();

        
        let temp_ui = BUTTONS
            .iter()
            .fold(Column::new(), |col, (label, action)| {
                col.push(button(*label).on_press(*action))
            })
            .push(progress_bar(0.0..=100.0, self.volume))
            .push(text(
                match &self.playback_sink {
                    Some(sink) => sink.get_pos(),
                    None => Duration::new(5, 0),
                }
                .as_secs(),
            ))
            //Get filenames and display as text
            .push(files.iter().fold(Column::new(), |col, file| {
                col.push(text(file.clone()))
            }),
            );
                            

        temp_ui.into()
    }

    //TODO: Only track time when source is playing
    pub fn subscription(&self) -> Subscription<AudioAction> {
        println!("x");
        time::every(Duration::from_secs(1)).map(|_instant| AudioAction::PlaybackTick)
        // Update every second
    }

    pub fn update(&mut self, message: AudioAction) -> Task<AudioAction> {
        match message {
            AudioAction::LoadAudio => {
                self.load_audio();
                Task::none()
            }
            AudioAction::TogglePlayPause => {
                if let Some(sink) = &self.playback_sink {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                Task::none()
            }
            AudioAction::SliderPositionChanged(value) => {
                self.volume = value;
                Task::none()
            }
            AudioAction::UpdatePlaybackPosition(value) => {
                Task::none()
            }
            AudioAction::PlaybackTick => {
                self.update_playback_position();
                Task::none()
            }
            AudioAction::Test => {
                Task::none()
            },


            _ => {
                Task::none()
            }
        }
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
            let _ = self.update(AudioAction::SliderPositionChanged(
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
}
