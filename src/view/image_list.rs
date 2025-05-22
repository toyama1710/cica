use crate::{
    message::Message,
    model::{CicaModel, ImgFileStatus},
    view::thumbnail::*,
};
use iced::{
    widget::{row, scrollable, text, Column},
    Border, Color, Element, Theme,
};

fn hidden_scrollbar_style(_theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
    scrollable::Style {
        container: iced::widget::container::Style::default(),
        vertical_rail: scrollable::Rail {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border: Border::default(),
            },
        },
        horizontal_rail: scrollable::Rail {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: iced::Border::default(),
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border: Border::default(),
            },
        },
        gap: None,
    }
}

pub fn sidebar_image_list(model: &CicaModel) -> Element<Message> {
    let images = model
        .images
        .iter()
        .rev()
        .map(|img| {
            let thumbnail = thumbnail(img);
            match img {
                ImgFileStatus::Loading(p) => {
                    row![thumbnail, text(p.file_name().unwrap().to_str().unwrap()),]
                }
                ImgFileStatus::NotImage((p, msg)) => {
                    row![
                        thumbnail,
                        text(p.file_name().unwrap().to_str().unwrap()),
                        text(msg),
                    ]
                }
                ImgFileStatus::IOerror((p, msg)) => {
                    row![
                        thumbnail,
                        text(p.file_name().unwrap().to_str().unwrap()),
                        text(msg),
                    ]
                }
                ImgFileStatus::Image(img) => {
                    row![thumbnail, text(img.filename.clone()),]
                }
            }
            .into()
        })
        .collect();
    scrollable(Column::from_vec(images))
        .style(hidden_scrollbar_style)
        .into()
}
