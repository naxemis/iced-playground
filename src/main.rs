// todo(app/view): Return the screen view as Element<screen::Message> without mapping.

// todo(app/update): Reset restores the initial state.
// todo(app/update): Clicked sets the text to "Working..." and raises the work flag (initially without a Task).
// todo(app/update): Finished sets the result text and clears the work flag.
// todo(app/update): Optional - guard Reset while work is in progress (the view already blocks it).

// todo(screen/task): Add a pub async fn that awaits tokio::time::sleep and returns the text "Finished!".
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
