use iced::{Element, Task};
use iced::widget::{text, toggler, button};
use crate::utils;

#[derive(Default)]
pub struct State {
   scan: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Scan,
    Scanned,
}

pub fn view(state: &State) -> Element<'_, Message> {
    button("Scan library").on_press(Message::Scan)
        .into()


}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Scan => Task::perform(
            async {
                utils::Util::scan_library().await;
                Message::Scanned
            },
            |msg| msg,
        ),
        Message::Scanned => {
            println!("Message::Scanned");
            Task::none()
        }
    }
}
