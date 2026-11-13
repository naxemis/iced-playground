// todo(main): Declare the app and screen modules.
// todo(app): Add a pub struct App without fields for now, with pub methods new, update and view.
// todo(app): New returns App; update and view are placeholders using a temporary message type (for example the unit type).
// todo(main): Return iced::Result from main and call iced::application(App::new, App::update, App::view).run().
// todo(main): Check with cargo run that an empty window opens (every next todo is checked the same way).

// todo(screen/state): Add a pub struct holding the text and an explicit "work in progress" flag (enum Idle/Working/Done or a separate field).
// todo(screen/state): Make the fields pub, because App::update changes them directly.
// todo(screen/state): Never infer the work state by comparing the text with "Working...".
// todo(app/new): Store the screen state in App; new builds it with the text "Nothing happened yet" and no work in progress.

// todo(screen/message): Add a pub enum Message with variants Clicked, Finished(String) and Reset.
// todo(screen/message): Derive Clone and Debug (required by Iced).
// todo(screen/message): Only Finished carries a payload (the result text).
// todo(app): Replace the temporary message type with screen::Message in update and view; there is no App-level Message or wrapper.

// todo(screen/view): Add a pub view that builds a column with the text and two buttons (start, reset).
// todo(screen/view): Give a button a Message only when no work is in progress; without a Message it is disabled.
// todo(app/view): Return the screen view as Element<screen::Message> without mapping.

// todo(app/update): Reset restores the initial state.
// todo(app/update): Clicked sets the text to "Working..." and raises the work flag (initially without a Task).
// todo(app/update): Finished sets the result text and clears the work flag.
// todo(app/update): Optional - guard Reset while work is in progress (the view already blocks it).

// todo(screen/task): Add a pub async fn that awaits tokio::time::sleep and returns the text "Finished!".
// todo(app/update): Clicked returns Task::perform with the future and a mapper to Message::Finished.
// todo(app/update): All other branches return Task::none(); the signature changes from () to Task<Message>.

fn main() {
    println!("Hello, world!");
}