mod message;
mod model;
mod ui;

use model::App;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
