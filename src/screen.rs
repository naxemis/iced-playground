pub enum ProgressStates {
    Idle,
    Working,
    Done,
}

pub struct Screen {
    pub current_text: String,
    pub progress_phase: ProgressStates,
}
