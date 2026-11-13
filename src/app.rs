use crate::screen::Screen;
use iced::Element;

pub enum Message {}

pub struct App {
    _screen: Screen,
}

impl App {
    pub fn new() -> Self {
        Self {
            _screen: Screen::new(),
        }
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        iced::widget::text("placeholder").into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
