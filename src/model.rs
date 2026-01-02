use crate::message::Message;

#[derive(Default)]
pub struct App {
    pub count: i32,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }
}

