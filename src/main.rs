use cica::update::update;
use cica::view::view;
use cica::{message::Message, model::CicaModel};
use iced::{
    application,
    keyboard::{
        self,
        key::{self, Named},
    },
    Size, Subscription, Task,
};

pub fn main() -> iced::Result {
    application("CiCA - CiCA is Color Analyzer", update, view)
        .theme(|_value: &CicaModel| iced::Theme::Dark)
        .window(iced::window::Settings {
            min_size: Some(Size {
                height: 400.,
                width: 600.,
            }),
            size: Size {
                height: 500.,
                width: 800.,
            },
            ..Default::default()
        })
        .subscription(|_model| {
            Subscription::batch(vec![iced::keyboard::on_key_press(|key, modifiers| {
                if modifiers.control()
                    && key.as_ref() == keyboard::Key::Named(keyboard::key::Named::Enter)
                {
                    Some(Message::EvalExprRequested)
                } else {
                    None
                }
            })])
        })
        .run_with(|| {
            (
                CicaModel {
                    ..Default::default()
                },
                Task::none(),
            )
        })
}
