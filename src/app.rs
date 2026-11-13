use crate::screen;
use crate::screen::Screen;
use iced::Element;

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
