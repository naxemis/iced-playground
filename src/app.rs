use crate::screen::{Message, ProgressStates, Screen};
use iced::Element;

pub struct App {
    screen: Screen,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Clicked => {
                self.screen.current_text = String::from("Working...");
                self.screen.progress_phase = ProgressStates::Working;
            }
            Message::Finished => {
                self.screen.current_text = String::from("Finished!");
                self.screen.progress_phase = ProgressStates::Finished;
            }
            Message::Reset => {
                if matches!(self.screen.progress_phase, ProgressStates::Finished) {
                    self.screen.current_text = String::from("Nothing happened yet.");
                    self.screen.progress_phase = ProgressStates::Idle;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.screen.view()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
