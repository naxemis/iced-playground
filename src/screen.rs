pub enum ProgressStates {
    Idle,
    Working,
    Done,
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
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
