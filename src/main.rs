use iced::widget::{
    button, column, container, row, scrollable, text, vertical_rule, Column, Container, Image, Row,
    Text,
};
use iced::{application, Alignment, Element, Length, Settings, Subscription, Task, Theme};
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct CicaModel {
    images: Vec<ImageStub>,
    selected_image_idx: Option<usize>,
    active_main_tab: MainTab,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImageStub {
    id: usize,
    path: PathBuf,
    filename: String,
}

#[derive(Debug, Clone, Default)]
pub enum MainTab {
    #[default]
    ImageView,
    RepresentativeColors,
    ColorHistgrams,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddImageClicked,
    ImageSelected(usize),
    TabSelected(MainTab),
}

fn update(value: &mut CicaModel, message: Message) -> Task<Message> {
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

fn view(model: &CicaModel) -> Row<Message> {
    let add_button = button("+ add image")
        .on_press(Message::AddImageClicked)
        .width(Length::Fill);
    let images = model
        .images
        .iter()
        .enumerate()
        .fold(Column::new(), |col, (idx, info)| {
            col.push(button(column![text(info.filename.clone()),]))
        });

    row![add_button, images]
}

pub fn main() -> iced::Result {
    application("CiCA - CiCA is Color Analyzer", update, view)
        .theme(|value: &CicaModel| iced::Theme::Dark)
        .run_with(|| {
            (
                CicaModel {
                    images: vec![
                        ImageStub {
                            id: 0,
                            path: "/usr/images/super_image".into(),
                            filename: "a.png".to_string(),
                        },
                        ImageStub {
                            id: 1,
                            path: "/usr/images/hyper_image".into(),
                            filename: "Axjfe.png".to_string(),
                        },
                    ],
                    ..Default::default()
                },
                Task::none(),
            )
        })
}
