#![allow(unused_imports, unused_braces)]
use iced::{Element};
use iced::Length;
use std::collections::HashMap;
use iced::widget::{image, container, button, slider, text, Column, Row, Button, progress_bar};
use lazy_static::lazy_static;
use iced::Color;
use iced::widget::Container;
use iced::widget::image::{Handle, Image};
use iced::Renderer;
use crate::app::state::audio::AudioState; // Reference the state controller
use crate::app::message::Audio;
use crate::app::view::playlist;
use iced::ContentFit;

impl AudioState {

    pub fn view(&self) -> Element<Audio> {
        // let v = Image::new(
        //             "C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/Stop.png".to_string()
        // );
        // let y:Button<Audio> = button(v).on_press(Audio::Test);


        //Playback controls
        let controls = Column::new()
            .push(Self::playback_controls()) //Already is a row
            .push(slider(0.0..=1.0, self.volume, Audio::Volume).step(0.01).width(200))
            .push(progress_bar(0.0..=self.song_duration(), self.current_pos))
            .push(button("Song duration").on_press(Audio::Duration));

        //Display songs in directory as playable buttons
        let file_list = Column::new()
            .push(button("Load files").on_press(Audio::ShowFiles))
            .push(self.files_as_buttons());

        //Top level container
        Column::new()
            .push(controls)
            .push(file_list)
            .into()

    }

    // Your `playback_controls` function with images
    pub fn playback_controls() -> Row<'static, Audio> {

        let playback_controls = BUTTONS.iter().fold(Row::new(), |row, (label, action)| {
            // Create a button with the image that will fit inside the button's size
            let image = Image::new(label.to_string())
                // .content_fit(ContentFit::Contain)
                .width(Length::Shrink)
                .height(Length::Shrink);
            

            row.push(
                Button::new(image)
                .on_press(action.clone())
                .style(MyButtonStyle))
        });

        playback_controls.into()
    }
}

//IS THIS REALLY STUPID? PROBABLY BUT THE EARLIER ONE WAS EVEN WORSE. KILL ME
pub const BUTTONS: &[(&'static str, Audio); 6] = &[
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/pause.png", Audio::Load),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/pause.png", Audio::Stop),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/pause.png", Audio::TogglePlayPause),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/play.png", Audio::Pause),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/prev.png", Audio::Prev),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/next.png", Audio::Next),
];

struct MyButtonStyle;

impl MyButtonStyle {
    pub fn style() -> button::Style {
        button::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
            text_color: Color::WHITE,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }
}

