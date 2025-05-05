use crate::message::Message;
use crate::model::CicaModel;
use crate::view::*;
use iced::widget::{
    button, column, container, row, scrollable, text, vertical_rule, Column, Container, Image, Row,
    Text,
};
use iced::Length;

pub fn view(model: &CicaModel) -> Row<Message> {
    let add_button = button("+ add")
        .on_press(Message::AddImageClicked)
        .width(Length::Fill);
    let side_images = sidebar_image_list(model);

    row![column![
        add_button,
        text(format!("{}", model.loading_files_count).to_owned()),
        scrollable(Column::from_vec(side_images))
    ]]
}
