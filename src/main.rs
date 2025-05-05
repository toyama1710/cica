use cica::model::CicaModel;
use cica::update::update;
use cica::view::view;
use iced::{application, Task};

pub fn main() -> iced::Result {
    application("CiCA - CiCA is Color Analyzer", update, view)
        .theme(|_value: &CicaModel| iced::Theme::Dark)
        .run_with(|| {
            (
                CicaModel {
                    ..Default::default()
                },
                Task::none(),
            )
        })
}
