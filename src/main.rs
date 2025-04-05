use iced::{
    Element, Length, Pixels, Size, Subscription, Task, Theme,
    font::{self, Family},
    keyboard,
    time::{Duration, Instant},
    widget::{Text, button, column, container, row, text},
    window,
};
use pomodori_manager::PomodoriManager;

mod pomodori_manager;
mod timer_manager;

pub fn main() -> iced::Result {
    iced::application("Pomodorable", Pomodorable::update, Pomodorable::view)
        .theme(Pomodorable::theme)
        .settings(settings())
        .window(window_settings())
        .subscription(Pomodorable::subscription)
        .run_with(move || Pomodorable::new())
}

fn settings() -> iced::Settings {
    iced::Settings {
        id: Some("pomodorable".to_string()),
        default_font: font::Font {
            family: Family::Monospace,
            ..font::Font::default()
        },
        default_text_size: Pixels::from(16.0),
        antialiasing: true,
        fonts: vec![],
    }
}

fn window_settings() -> iced::window::Settings {
    window::Settings {
        transparent: false,
        decorations: false,
        resizable: false,
        position: window::Position::Centered,
        size: Size {
            width: 200.0,
            height: 300.0,
        },
        maximized: true,
        fullscreen: false,
        level: window::Level::AlwaysOnTop,
        // exit_on_close_request: true,
        // platform_specific: window::settings::PlatformSpecific {
        //     application_id: "mouser".to_string(),
        //     override_redirect: true
        // },
        ..window::Settings::default()
    }
}

#[derive(Default)]
enum State {
    #[default]
    Idle,
    Running {
        last_tick: Instant,
    },
}

// #[derive(Default)]
pub struct Pomodorable {
    theme: Theme,
    pomo: PomodoriManager,
    state: State,
    duration: Duration,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CloseWindow,
    Tick(Instant),
    ToggleTimer,
    ResetTimer,
    SkipInterval,
}

impl Pomodorable {
    fn new() -> (Pomodorable, Task<Message>) {
        let mouser = Pomodorable {
            theme: Theme::GruvboxDark,
            pomo: PomodoriManager::new(),
            state: State::Idle,
            duration: Duration::default(),
        };

        (mouser, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", message);

        match message {
            Message::CloseWindow => window::get_latest().and_then(window::close),
            Message::Tick(t) => {
                if let State::Running { last_tick } = &mut self.state {
                    self.duration += t - *last_tick;
                    *last_tick = t;

                    if self.duration > self.pomo.get_current_interval() {
                        self.pomo.next();
                        self.duration = Duration::default();
                    }
                }
                Task::none()
            }
            Message::ToggleTimer => {
                self.state = match self.state {
                    State::Idle => State::Running {
                        last_tick: Instant::now(),
                    },
                    State::Running { .. } => State::Idle,
                };

                Task::none()
            }
            Message::ResetTimer => {
                self.pomo.reset();
                self.state = State::Idle;
                self.duration = Duration::default();

                Task::none()
            }
            Message::SkipInterval => {
                self.pomo.next();
                self.duration = Duration::default();

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let target = self.pomo.get_current_interval().as_secs();
        let seconds = self.duration.as_secs();

        container(column![
            text!(
                "{} - {}",
                if self.pomo.is_break() {
                    "Break time!"
                } else {
                    "Work work"
                },
                self.pomo.get_interval_count()
            ),
            container(
                text!(
                    "{:0>2}:{:0>2}",
                    (((target - seconds) % HOUR) / MINUTE),
                    (MINUTE - seconds) % MINUTE,
                )
                .size(40),
            )
            .center(Length::FillPortion(3)),
            container(
                row![
                    button(match self.state {
                        State::Idle => "Start",
                        State::Running { .. } => "Stop",
                    })
                    .on_press(Message::ToggleTimer),
                    button("Reset").on_press(Message::ResetTimer),
                    button("Skip").on_press(Message::SkipInterval)
                ]
                .spacing(4)
            )
            .center(Length::FillPortion(2))
        ])
        .center(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = match self.state {
            State::Idle => Subscription::none(),
            State::Running { .. } => {
                iced::time::every(iced::time::milliseconds(100)).map(Message::Tick)
            }
        };

        let hotkeys = keyboard::on_key_press(|key, _modifiers| {
            use keyboard::key;
            match key.as_ref() {
                keyboard::Key::Named(key::Named::Escape) => Some(Message::CloseWindow),
                keyboard::Key::Named(key::Named::Space) => Some(Message::ToggleTimer),
                keyboard::Key::Character("r") => Some(Message::ResetTimer),

                _ => None,
            }
        });

        Subscription::batch(vec![hotkeys, tick])
    }
}
