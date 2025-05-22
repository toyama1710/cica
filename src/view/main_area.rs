use crate::message::Message;
use crate::model::{CicaModel, MainTab};
use iced::widget::{button, column, row, Button};
use iced::{border, Border, Color, Element, Theme};

pub fn view(model: &CicaModel) -> Element<Message> {
    let inactive_style = |_theme: &Theme, _status: iced::widget::button::Status| button::Style {
        border: Border {
            radius: 0.0.into(),
            color: Color::from_rgba8(0, 0, 0, 0.8),
            width: 0.5,
            ..Default::default()
        },
        text_color: Color::from_rgb8(160, 160, 160),
        background: Some(Color::from_rgb8(50, 50, 50).into()),
        ..Default::default()
    };

    let active_style = |_theme: &Theme, _status: iced::widget::button::Status| button::Style {
        border: Border {
            radius: 0.0.into(),
            ..Default::default()
        },
        text_color: Color::from_rgb8(255, 255, 255),
        ..Default::default()
    };

    let (image_view_button, representative_colors_button, color_histgrams_button) =
        match &model.active_main_tab {
            MainTab::ImageView => (
                _tab_button(MainTab::ImageView, active_style),
                _tab_button(MainTab::RepresentativeColors, inactive_style),
                _tab_button(MainTab::ColorHistgrams, inactive_style),
            ),
            MainTab::RepresentativeColors => (
                _tab_button(MainTab::ImageView, inactive_style),
                _tab_button(MainTab::RepresentativeColors, active_style),
                _tab_button(MainTab::ColorHistgrams, inactive_style),
            ),
            MainTab::ColorHistgrams => (
                _tab_button(MainTab::ImageView, inactive_style),
                _tab_button(MainTab::RepresentativeColors, inactive_style),
                _tab_button(MainTab::ColorHistgrams, active_style),
            ),
        };

    column![row![
        image_view_button,
        representative_colors_button,
        color_histgrams_button
    ],]
    .into()
}

fn _tab_button<'a>(
    label: MainTab,
    style: impl Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a,
) -> Button<'a, Message> {
    match label {
        MainTab::ImageView => button("Image View")
            .on_press(Message::TabSelected(MainTab::ImageView))
            .style(style),
        MainTab::RepresentativeColors => button("Representative")
            .on_press(Message::TabSelected(MainTab::RepresentativeColors))
            .style(style),
        MainTab::ColorHistgrams => button("Histgram")
            .on_press(Message::TabSelected(MainTab::ColorHistgrams))
            .style(style),
    }
}
