use iced::Element;
use iced::widget::{button, column, text};

#[derive(Clone, Debug)]
pub enum Message {
    Clicked,
    Finished,
    Reset,
}

pub enum ProgressStates {
    Idle,
    Working,
    Finished,
}

pub struct Screen {
    pub current_text: String,
    pub progress_phase: ProgressStates,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            current_text: String::from("Nothing happened yet."),
            progress_phase: ProgressStates::Idle,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let buttons = match &self.progress_phase {
            ProgressStates::Idle => button("Start").on_press_maybe(Some(Message::Clicked)),
            ProgressStates::Working => button("Working...").on_press_maybe(None),
            ProgressStates::Finished => button("Reset").on_press_maybe(Some(Message::Reset)),
        };

        column![text(&self.current_text), buttons,].into()
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
