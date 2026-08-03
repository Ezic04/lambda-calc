mod app;
mod dsl;
mod macros;
mod repl;
mod spreadsheet;

use crate::app::App;

pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
