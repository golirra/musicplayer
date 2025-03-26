#![allow(unused_imports, unused_braces)]
use iced::{Element};
use iced::Length;
use iced::Background;
use iced::widget::button::{Style, Status};
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

        //Playback controls
        let controls = Row::new()
            .push(Self::playback_controls()) //is a row
            .push(slider(0.0..=1.0, self.volume, Audio::Volume).step(0.01).width(200))
            .push(progress_bar(0.0..=self.song_duration(), self.current_pos))
            .push(button("duration").on_press(Audio::Duration));

        //Top level container
        Column::new()
            .push(controls)
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
                // .style(MyButtonStyle::style_fn())
                .on_press(action.clone())
                // .style(MyButtonStyle::style())
                )
        });

        playback_controls.into()
    }
}

//IS THIS REALLY STUPID? PROBABLY BUT THE EARLIER ONE WAS EVEN WORSE. KILL ME
pub const BUTTONS: &[(&'static str, Audio); 5] = &[
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/pause.png", Audio::Stop),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/pause.png", Audio::TogglePlayPause),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/play.png", Audio::Pause),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/prev.png", Audio::Prev),
    ("C:/Users/webbs/programming/cs/rust/musicplayer/assets/playback/next.png", Audio::Next),
];


struct MyButtonStyle;

impl MyButtonStyle {
    fn style_fn<'a>() -> impl Fn(&(), Status) -> button::Style + 'a {
        // This function returns a closure that applies the button style based on its status
        move |_theme: &(), status: Status| {
            match status {
                Status::Hovered => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.3, 0.8, 0.3))),
                    text_color: Color::WHITE,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                },
                Status::Active => button::Style {
                    background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
                    text_color: Color::WHITE,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                },
                Status::Pressed => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
                    text_color: Color::WHITE,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                },
                Status::Disabled => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
                    text_color: Color::WHITE,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                },
            }
        }
    }
}

