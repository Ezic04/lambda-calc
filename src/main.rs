mod app;
mod dsl;
mod macros;
mod repl;
mod selection;
mod spreadsheet;

use crate::app::App;

pub fn main() -> iced::Result {
    iced::application::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .run()
}
