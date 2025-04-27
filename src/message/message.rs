use crate::message::file_io;
use crate::model::*;
use iced::widget::Image;
use iced::Task;
use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Message {
    AddImageClicked,
    ImageSelected(usize),
    TabSelected(MainTab),
    FilesOpen(Vec<FileHandle>),
}

pub fn update(value: &mut CicaModel, message: Message) -> Task<Message> {
    match message {
        Message::AddImageClicked => {
            #[cfg(debug_assertions)]
            println!("Add Image button clicked");

            Task::perform(file_io::pick_files(), Message::FilesOpen)
        }
        Message::ImageSelected(idx) => {
            println!("Image {} selected", idx);
            value.selected_image_idx = Some(idx);
            Task::none()
        }
        Message::TabSelected(tab) => {
            println!("Tab {:?} selected", tab);
            value.active_main_tab = tab;
            Task::none()
        }
        Message::FilesOpen(handles) => {
            for h in handles {
                value.images.push(ImageStub {
                    id: 0,
                    path: h.path().into(),
                    filename: h.file_name(),
                })
            }
            Task::none()
        }
    }
}
