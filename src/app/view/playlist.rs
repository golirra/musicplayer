use std::sync::Arc;
use std::fs;
use iced::Element;
use iced::widget::{button, Column};
use crate::app::message::Audio;

#[derive(Default)]
pub struct Playlist {
    files: Vec<Arc<String>>,
}

impl Playlist {

    pub fn new() -> Self {
        Self {
            files: vec![],
        }
    }

    pub fn view(&self) -> Column<Audio> {
        self.files_as_buttons().push(button("Load").on_press(Audio::ShowFiles))
    }

    pub fn files_as_buttons(&self) -> Column<Audio> {
         self.files
            .iter()
            .fold(Column::new(), |column, filename| {
                column.push(button(filename.as_str()).on_press(Audio::Play(filename.clone())))
            })
               
    }
    pub fn get_filenames_in_directory() -> Vec<String> {
        fs::read_dir("./")
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect()
    }
}

