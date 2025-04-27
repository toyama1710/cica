use cica::message::update;
use cica::model::CicaModel;
use cica::model::ImageStub;
use cica::view::view;
use iced::{application, Task};

pub fn main() -> iced::Result {
    application("CiCA - CiCA is Color Analyzer", update, view)
        .theme(|_value: &CicaModel| iced::Theme::Dark)
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
