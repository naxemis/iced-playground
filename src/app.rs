use crate::screen;
use crate::screen::Screen;
use iced::Element;

// todo(app/update): Reset restores the initial state.
// todo(app/update): Clicked sets the text to "Working..." and raises the work flag (initially without a Task).
// todo(app/update): Finished sets the result text and clears the work flag.
// todo(app/update): Optional - guard Reset while work is in progress (the view already blocks it).

pub enum Message {}

pub struct App {
    screen: Screen,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }

    pub fn update(&mut self, _message: screen::Message) {}

    pub fn view(&self) -> Element<'_, screen::Message> {
        self.screen.view()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
