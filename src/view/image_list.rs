use crate::{
    message::Message,
    model::{CicaModel, ImgFileStatus},
    view::thumbnail::*,
};
use iced::{
    widget::{image, row, text},
    Alignment::Center,
    Element,
};

pub fn sidebar_image_list(model: &CicaModel) -> Vec<Element<Message>> {
    model
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
        .collect()
}
