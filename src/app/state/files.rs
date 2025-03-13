// file_state.rs
use std::fs;
use iced::{Task};
use crate::app::message::File;

#[derive(Default)]
pub struct FileState {
    files: Vec<String>,
}

impl FileState {
    pub fn new() -> Self {
        Self {
            files: vec!["file1.mp3".to_string(), "file2.mp3".to_string()],
        }
    }

    pub fn update(&mut self, message: File) -> Task<File> {
        match message {
            File::Load => {
                //TODO:
                Task::none()
            },
            File::Display => {
                //TODO:
                Task::none()
            },
            _ => {Task::none()},
        }
    }

    pub fn get_filenames_in_directory() -> Vec<String> {
        fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect()
    }
}
