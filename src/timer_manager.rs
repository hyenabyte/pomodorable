use std::time::Duration;

pub struct TimerManager {
    running: bool,
}

impl TimerManager {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub fn start(&mut self) {
        todo!()
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn is_running(&self) -> bool {
        todo!()
    }

    pub fn get_elapsed(&self) -> Duration {
        todo!()
    }

    pub fn pause(&mut self) {
        todo!()
    }
}
