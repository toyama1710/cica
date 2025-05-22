use crate::common::file_io::open_file;
use crate::common::file_io::pick_files;
use crate::message::*;
use crate::model::*;
use iced::Task;
use std::path::PathBuf;

pub fn update(model: &mut CicaModel, message: Message) -> Task<Message> {
    #[cfg(debug_assertions)]
    println!("{:?}", message);

    match message {
        Message::AddImageClicked => Task::perform(pick_files(), Message::FilesPicked),
        Message::TabSelected(tab) => {
            model.active_main_tab = tab;
            Task::none()
        }
        Message::FilesPicked(handles) => {
            let handles: Vec<PathBuf> = handles.iter().map(|p| p.path().to_path_buf()).collect();
            model.loading_files_count += handles.len();
            let mut tasks = vec![];
            for path in handles {
                let idx = model.images.len();
                model.images.push(ImgFileStatus::Loading(path.clone()));
                /*
                tasks.push(Task::perform(
                    open_file(path.clone(), idx),
                    Message::FileOpened,
                ));
                */
                tasks.push(Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || open_file(path.clone(), idx))
                            .await
                            .unwrap()
                            .await
                    },
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
        Message::ExprUpdated(action) => {
            model.expr_state.perform(action);
            Task::none()
        }
        Message::EvalExprRequested => Task::none(),
    }
}
