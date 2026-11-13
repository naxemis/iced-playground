#![windows_subsystem = "windows"]

use crate::app::App;

pub mod app;
pub mod screen;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Iced Playground")
        .window_size((640, 480))
        .run()
}
