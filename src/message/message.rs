use crate::model::*;
use iced::Task;

#[derive(Debug, Clone)]
pub enum Message {
    AddImageClicked,
    ImageSelected(usize),
    TabSelected(MainTab),
}

pub fn update(value: &mut CicaModel, message: Message) -> Task<Message> {
    match message {
        Message::AddImageClicked => {
            println!("Add Image button clicked");
            value.error_message = None;
        }
        Message::ImageSelected(idx) => {
            println!("Image {} selected", idx);
            value.selected_image_idx = Some(idx);
        }
        Message::TabSelected(tab) => {
            println!("Tab {:?} selected", tab);
            value.active_main_tab = tab;
        }
    }
    Task::none()
}
