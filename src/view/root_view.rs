use crate::message::Message;
use crate::model::CicaModel;
use iced::widget::{
    button, column, container, row, scrollable, text, vertical_rule, Column, Container, Image, Row,
    Text,
};
use iced::Length;

pub fn view(model: &CicaModel) -> Row<Message> {
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
