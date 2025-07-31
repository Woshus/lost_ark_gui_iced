//! List sales and navigate to sale details or editing
use iced::Element;
use iced::widget::{column, button, text};

// use crate::Sale;

#[derive(Debug, Clone)]
pub enum Message {
    SwapPage,
}

pub fn view() -> Element<'static, Message> {
    let hello = text("Hello World").size(50);
    let navigate = button("go back").on_press(Message::SwapPage);
    let interface = column![hello, navigate];
    interface.into()
}
