#![allow(dead_code)]
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use iced::{executor, time, Application};
use iced::Subscription;
use iced::{Fill, Element};
use iced::widget::{button, column, container, progress_bar, row, slider, text};
use iced::window;
use crate::playback;

use rodio::{Decoder, OutputStream, Sink, Source};

#[derive(Clone, Debug)]
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
}

pub struct AudioState {
    value: f32,
}

#[derive(Default)]
pub struct AudioPlaybackController {
    volume: f32,
    currentPosition: f32,
    playbackSink: Option<Sink>,
    _audioStream: Option<OutputStream>,
}

#[derive(Clone, Debug)]
enum PlayerMessage {
    ButtonPressed,
}

impl AudioPlaybackController {

    pub fn new() -> Self {
        Self {
            volume: 0.0,
            currentPosition: 0.0,
            playbackSink: None,
            _audioStream: None,
        }
    }
    
    pub fn view(&self) -> Element<AudioAction> {
        container(
            row![
                slider(0.0..=100.0, self.volume, AudioAction::SliderPositionChanged),
                progress_bar(0.0..=100.0, self.volume),
                text(
                    match &self.playbackSink {
                        Some(sink) => sink.get_pos(),
                        None => Duration::new(5, 0),
                    }.as_secs()
                ),
                container(
                    row![
                        button("Load audio").on_press(AudioAction::LoadAudio),
                        button("Stop").on_press(AudioAction::StopPlayback),
                        button("Play/Pause").on_press(AudioAction::TogglePlayPause),
                        button("Pause").on_press(AudioAction::PausePlayback),
                        button("Previous").on_press(AudioAction::PreviousTrack),
                        button("Next").on_press(AudioAction::NextTrack),
                        button("Random Next").on_press(AudioAction::RandomNextTrack),
                    ]
                    .spacing(1)
                )
            ]
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<AudioAction> {
        println!("Subscription started");
        time::every(Duration::from_secs(1)).map(|_instant| AudioAction::PlaybackTick) // Update every second
    }

    pub fn update(&mut self, message: AudioAction) {
        match message {
            AudioAction::LoadAudio => {
                self.load_audio();
            },
            AudioAction::TogglePlayPause => {
                if let Some(sink) = &self.playbackSink {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
            },
            AudioAction::SliderPositionChanged(value) => {
                self.volume = value;
            },
            AudioAction::UpdatePlaybackPosition(value) => {
            },
            AudioAction::PlaybackTick => {
                self.update_playback_position();
                println!("PlaybackTick");
            },
            _ => {}, 
        }
    }

    pub fn load_audio(&mut self) {
        if self.playbackSink.is_none() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let file = BufReader::new(File::open("song.mp3").unwrap());
            let source = Decoder::new(file).unwrap();

            sink.append(source);
            sink.set_volume(0.05);

            self._audioStream = Some(stream);
            self.playbackSink = Some(sink);
        }
    }

    pub fn update_playback_position(&mut self) {
        if let Some(sink) = &self.playbackSink {
            self.update(AudioAction::SliderPositionChanged(sink.get_pos().as_secs_f32()));
        }
    }

    pub fn play_audio(&mut self) {
        if let Some(sink) = &self.playbackSink {
            sink.play();
            self.volume = 51.0;
        }
    }

    pub fn pause_audio(&mut self) {
        if let Some(sink) = &self.playbackSink {
            sink.pause();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_playback() {
        let mut controller = AudioPlaybackController::new();
        controller.update(AudioAction::TogglePlayPause);
    }
}

