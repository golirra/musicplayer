use std::fs;
use std::sync::{Arc, Mutex};
use iced::Length;
use iced::Element;
use iced::widget::{Button, Column, button};
use iced::{Task};
use crate::app::message::{File, Audio};
use crate::Message;
use crate::app::state::audio::AudioState;

use crate::app::state::db::scanner;
use crate::app::state::db::scanner::Metadata;

#[derive(Default)]
pub struct FileState {
    pub files: Vec<(String, Metadata)>,
}

impl FileState {
    pub fn new() -> Self {
        Self {
            files: vec![],
        }
    }

    pub fn update(&mut self, message: File) -> Task<File> {
        match message {
            File::Load => {
                println!("File::Load");
                Task::none()
            },
            File::Duration => {
                Task::none()
            },
            File::Display => {
                self.files = scanner::get_paths_with_metadata().unwrap().into_iter().collect();
                Task::none()
            },
            File::Select(_path) => {
                Task::none()
            },
        }
    }

    pub fn get_filenames_in_directory() -> Vec<String> {
        fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect()
    }

    pub fn files_as_buttons(&self) -> Column<File> {
            self.files
                .iter()
                .fold(Column::new(), |column, tuple| {
                    column.push(
                        button(tuple.1.title.as_str())
                        .on_press(File::Select(tuple.0.clone()))
                        .height(Length::Shrink)
                        .width(Length::Shrink))
                })
        }
}
