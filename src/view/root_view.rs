use crate::message::Message;
use crate::model::CicaModel;
use crate::view::image_list::sidebar_image_list;
use iced::widget::{
    button, column, container, row, scrollable, text, text_editor, text_input, vertical_rule,
    Column, Container, Image, Row, Scrollable, Text,
};
use iced::{Element, Length};

pub fn view(model: &CicaModel) -> Element<Message> {
    let add_button = button("+ add")
        .on_press(Message::AddImageClicked)
        .width(Length::Fill);
    let side_images = sidebar_image_list(model);

    let main = crate::view::main_area::view(model);

    row![
        column![
            add_button,
            // text(format!("{}", model.loading_files_count).to_owned()),
            text_editor(&model.expr_state)
                .on_action(Message::ExprUpdated)
                .wrapping(text::Wrapping::Glyph)
                .height(300),
            side_images,
        ]
        .width(Length::Fixed(250.)),
        main
    ]
    .into()
}
