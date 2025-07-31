use iced::{Element};
use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    SwapPage,
}

pub struct Counter {
    value: i64,
}

impl Counter {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

pub fn view(counter: &Counter) -> Element<'_, Message> {
    let increment = button("+").on_press(Message::Increment);
    let decrement = button("-").on_press(Message::Decrement);

    let counter = text(counter.value).size(50);

    let navigate = button("next page").on_press(Message::SwapPage);

    let interface = column![increment, counter, decrement, navigate];
    interface.into()
}

pub fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => {
            counter.value +=1;
        }
        Message::Decrement => {
            counter.value -=1;
        }
        _ => {}
    }
}
