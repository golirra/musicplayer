use std::sync::Arc;
use std::fs;
use iced::Element;
use iced::widget::{button, Column};
use crate::app::message::{Audio, File};
use crate::app::message::Message;
use crate::FileState;
use rusqlite::Connection;


impl FileState {

    //Display songs in directory as playable buttons
    pub fn view(&self) -> Element<File> {
        let file_list = Column::new()
            .push(button("Load files").on_press(File::Display))
            .push(self.files_as_buttons());
        file_list.into()
    }
}
