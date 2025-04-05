use std::time::Duration;

const LONG_BREAK_INTERVAL: i32 = 4;
const MINUTE: u64 = 60;

#[derive(Default, Clone, Copy)]
pub enum PomodoroType {
    #[default]
    Working,
    Break,
    LongBreak,
}

pub struct PomodoriManager {
    interval_count: i32,
    current: PomodoroType,
}

impl PomodoriManager {
    pub fn new() -> Self {
        Self {
            interval_count: 0,
            current: PomodoroType::default(),
        }
    }

    /// Resets interval
    pub fn reset(&mut self) {
        self.interval_count = 0;
        self.current = PomodoroType::Working;
    }

    pub fn get_interval_count(&self) -> i32 {
        self.interval_count
    }

    /// Get length of the current interval as a Duration
    pub fn get_current_interval(&self) -> Duration {
        match self.current {
            PomodoroType::Working => Duration::from_secs(25 * MINUTE),
            PomodoroType::Break => Duration::from_secs(5 * MINUTE),
            PomodoroType::LongBreak => Duration::from_secs(30 * MINUTE),
        }
    }

    /// Is the current interval a break?
    pub fn is_break(&self) -> bool {
        match self.current {
            PomodoroType::Working => false,
            PomodoroType::Break => true,
            PomodoroType::LongBreak => true,
        }
    }

    /// Move on to next interval
    pub fn next(&mut self) {
        self.current = match self.current {
            PomodoroType::Working if (self.interval_count + 1) % LONG_BREAK_INTERVAL == 0 => {
                PomodoroType::LongBreak
            }
            PomodoroType::Working => PomodoroType::Break,
            _ => {
                self.interval_count += 1;
                PomodoroType::Working
            }
        };
    }
}
