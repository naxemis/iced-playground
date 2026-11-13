use iced::Element;

pub enum Message {}

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        iced::widget::text("iced-playground").into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
