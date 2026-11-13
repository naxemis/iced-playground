// todo(app/update): Clicked returns Task::perform with the future and a mapper to Message::Finished.
// todo(app/update): All other branches return Task::none(); the signature changes from () to Task<Message>.

use crate::app::App;

pub mod app;
pub mod screen;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Iced Playground")
        .window_size((640, 480))
        .run()
}
