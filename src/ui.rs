use iced::widget::{button, column, text};
use iced::{Center, Element};

use crate::message::Message;
use crate::model::App;

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Hello, World!").size(32),
            text(format!("Counter: {}", self.count)).size(24),
            button("+ Increment").on_press(Message::Increment),
            button("- Decrement").on_press(Message::Decrement),
        ]
        .spacing(20)
        .align_x(Center)
        .into()
    }
}
