mod button;
use crate::button::ButtonApp;
fn main() -> iced::Result {
   
    iced::run("My program!", ButtonApp::update, ButtonApp::view)
}
