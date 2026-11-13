// todo(main): Open an empty window with iced::run, using a placeholder update and view (so every next todo can be checked with cargo run).

// todo(state): Hold the text and an explicit "work in progress" flag (enum Idle/Working/Done or a separate field).
// todo(state): Never infer the work state by comparing the text with "Working...".
// todo(state): Initial value is the text "Nothing happened yet" and no work in progress.
// todo(state): Implement Default (derive or manual), because iced::run builds the state from it.

// todo(message): Add variants Clicked, Finished(String) and Reset.
// todo(message): Derive Clone and Debug (required by Iced).
// todo(message): Only Finished carries a payload (the result text).

// todo(view): Build a column with the text and two buttons (start, reset).
// todo(view): Give a button a Message only when no work is in progress; without a Message it is disabled.

// todo(update): Reset restores the initial state.
// todo(update): Clicked sets the text to "Working..." and raises the work flag (initially without a Task).
// todo(update): Finished sets the result text and clears the work flag.
// todo(update): Optional - guard Reset while work is in progress (the view already blocks it).
// todo(main): Wire update and view into iced::run in place of the placeholders.

// todo(task): Write an async fn that awaits tokio::time::sleep and returns the text "Finished!".
// todo(update): Clicked returns Task::perform with the future and a mapper to Message::Finished.
// todo(update): All other branches return Task::none(); the signature changes from () to Task<Message>.

fn main() {
    println!("Hello, world!");
}