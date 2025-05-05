use crate::common::file_io::open_file;
use crate::common::file_io::pick_files;
use crate::message::*;
use crate::model::*;
use iced::Task;
use std::path::PathBuf;

pub fn update(model: &mut CicaModel, message: Message) -> Task<Message> {
    match message {
        Message::AddImageClicked => {
            #[cfg(debug_assertions)]
            println!("Add Image button clicked");

            Task::perform(pick_files(), Message::FilesPicked)
        }
        Message::TabSelected(tab) => {
            println!("Tab {:?} selected", tab);
            model.active_main_tab = tab;
            Task::none()
        }
        Message::FilesPicked(handles) => {
            let handles: Vec<PathBuf> = handles.iter().map(|p| p.path().to_path_buf()).collect();
            model.loading_files_count += handles.len();
            let mut tasks = vec![];
            for path in handles {
                model.images.push(ImgFileStatus::Loading(path.clone()));
                tasks.push(Task::perform(
                    open_file(path.clone(), model.images.len() - 1),
                    Message::FileOpened,
                ));
            }
            Task::batch(tasks)
        }
        Message::FileOpened((f, idx)) => {
            model.loading_files_count -= 1;
            model.images[idx] = f;
            Task::none()
        }
    }
}
