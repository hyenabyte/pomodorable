use rand::seq::IndexedRandom;
use std::time::Duration; // 0.9.0

const MINUTE: u64 = 60;

#[derive(Default, Clone, Copy)]
pub enum State {
    #[default]
    Ready,
    Focus,
    Break,
    LongBreak,
    Finished,
}

#[derive(Default, Clone, Copy)]
pub struct Settings {
    pub focus_length: u64,
    pub short_break_length: u64,
    pub long_break_length: u64,
    pub long_break_interval: i32,
    pub interval_target: i32,
}

impl Settings {
    fn default() -> Self {
        Self {
            focus_length: 25,
            short_break_length: 5,
            long_break_length: 30,
            long_break_interval: 4,
            interval_target: 10,
        }
    }
}

pub struct Pomodori {
    interval_count: i32,
    state: State,
    settings: Settings,
    current_quote: String,
}

impl Pomodori {
    pub fn new() -> Self {
        Self {
            interval_count: 0,
            state: State::default(),
            settings: Settings::default(),
            current_quote: Self::pick_quote(State::default()),
        }
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Self::new()
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_interval_count(&self) -> i32 {
        self.interval_count
    }

    pub fn get_interval_target(&self) -> i32 {
        self.settings.interval_target
    }

    pub fn get_quote(&self) -> String {
        self.current_quote.clone()
    }

    /// Get length of the current interval as a Duration
    pub fn get_current_interval(&self) -> Duration {
        match self.state {
            State::Focus => Duration::from_secs(self.settings.focus_length * MINUTE),
            State::Break => Duration::from_secs(self.settings.short_break_length * MINUTE),
            State::LongBreak => Duration::from_secs(self.settings.long_break_length * MINUTE),
            State::Finished => Duration::ZERO,
            State::Ready => Duration::from_secs(self.settings.focus_length * MINUTE),
        }
    }

    /// Resets interval
    pub fn reset(&mut self) {
        self.interval_count = 0;
        self.set_state(State::Ready);
    }

    /// Move on to next interval
    pub fn next(&mut self) {
        let new_state = match self.state {
            State::Ready => State::Focus,

            // If finished do nothing
            State::Finished => State::Finished,

            // If interval target reached finish
            State::Focus if (self.interval_count + 1) >= self.settings.interval_target => {
                State::Finished
            }

            // If interval count is divisible by the long break interval, go to long break
            State::Focus if (self.interval_count + 1) % self.settings.long_break_interval == 0 => {
                State::LongBreak
            }

            // If working go to break
            State::Focus => State::Break,

            // Only increase interval count when finishing a break
            _ => {
                self.interval_count += 1;
                State::Focus
            }
        };

        self.set_state(new_state);
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
        self.current_quote = Self::pick_quote(state);
    }

    fn pick_quote(state: State) -> String {
        let ready_quotes = vec![
            "Welcome to Pomodorable",
            "Let's work together!",
            "Hi friend",
            "Let's get some work done!",
        ];
        let focus_quotes = vec![
            "Work work",
            "Focus time",
            "Time to get stuff done!",
            "Time to be productive",
        ];
        let short_break_quotes = vec![
            "You deserve a short break",
            "Ahh break time",
            "Take five",
            "Remember to hydrate",
            "Remember to stretch",
        ];
        let long_break_quotes = vec![
            "Break time!",
            "Step away from the computer for a bit",
            "You deserve some rest",
            "Go and get a snack",
        ];
        let finished_quotes = vec![
            "All done, good job!",
            "Finished! Nice job!",
            "DONE! LOOK AT YOU GO!",
            "Mission complete!",
        ];

        let quote_bank = match state {
            State::Ready => ready_quotes,
            State::Focus => focus_quotes,
            State::Break => short_break_quotes,
            State::LongBreak => long_break_quotes,
            State::Finished => finished_quotes,
        };

        match quote_bank.choose(&mut rand::rng()) {
            Some(s) => s,
            None => "Uh oh you should not be seeing this O:",
        }
        .to_string()
    }
}
