use iced::{
    Element, Length, Pixels, Size, Subscription, Task,
    font::{self, Family},
    keyboard,
    time::{Duration, Instant},
    widget::{button, column, container, progress_bar, row, svg, text, tooltip},
    window,
};

use pomodori::Pomodori;

mod pomodori;
mod theme;
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
        decorations: true,
        resizable: false,
        position: window::Position::Centered,
        size: Size {
            width: 250.0,
            height: 350.0,
        },
        maximized: true,
        fullscreen: false,
        // level: window::Level::AlwaysOnTop,
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
    theme: iced::Theme,
    pomodori: Pomodori,
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
    Settings,
}

impl Pomodorable {
    fn new() -> (Pomodorable, Task<Message>) {
        let mouser = Pomodorable {
            theme: theme::Everforest::light_medium(),
            pomodori: Pomodori::with_settings(pomodori::Settings {
                focus_length: 2,
                short_break_length: 1,
                long_break_length: 2,
                long_break_interval: 3,
                interval_target: 5,
            }),
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
                if let pomodori::State::Finished = self.pomodori.get_state() {
                    // Pomodori finished stop the timer
                    self.state = State::Idle;
                    self.duration = Duration::default();
                } else if let State::Running { last_tick } = &mut self.state {
                    self.duration += t - *last_tick;
                    *last_tick = t;

                    if self.duration > self.pomodori.get_current_interval() {
                        self.pomodori.next();
                        self.duration = Duration::default();
                    }
                }
                Task::none()
            }
            Message::ToggleTimer => {
                self.state = match self.state {
                    State::Idle => {
                        match self.pomodori.get_state() {
                            pomodori::State::Ready => {
                                self.pomodori.next();
                                ()
                            }
                            pomodori::State::Finished => {
                                self.duration = Duration::default();
                                self.pomodori.reset();
                                self.pomodori.next();
                                ()
                            }
                            _ => (),
                        }

                        State::Running {
                            last_tick: Instant::now(),
                        }
                    }
                    State::Running { .. } => State::Idle,
                };

                Task::none()
            }
            Message::ResetTimer => {
                self.pomodori.reset();
                self.state = State::Idle;
                self.duration = Duration::default();

                Task::none()
            }
            Message::SkipInterval => {
                match self.pomodori.get_state() {
                    pomodori::State::Finished => (),
                    _ => {
                        self.pomodori.next();
                        self.duration = Duration::default();
                    }
                }

                Task::none()
            }
            Message::Settings => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        const SECOND: u128 = 1000;
        const MINUTE: u128 = 60 * SECOND;
        const HOUR: u128 = 60 * MINUTE;

        let target = self.pomodori.get_current_interval().as_millis();
        let millis = self.duration.as_millis();

        let progress = millis as f32 / target as f32;

        let seconds = ((target - millis) % MINUTE) / SECOND;
        let minutes = ((target - millis) % HOUR) / MINUTE;

        container(
            column![
                row![
                    // text!(
                    //     "{}/{}",
                    //     self.pomodori.get_interval_count(),
                    //     self.pomodori.get_interval_target()
                    // ),
                    container("Pomodorable v0.1")
                        .style(container::primary)
                        .padding(4)
                        .center_x(Length::Fill)
                ],
                column![
                    container(
                        text!(
                            "{}",
                            match self.pomodori.get_state() {
                                pomodori::State::Ready => "Ready",
                                pomodori::State::Focus => "Focus",
                                pomodori::State::Break => "Short break",
                                pomodori::State::LongBreak => "Long break",
                                pomodori::State::Finished => "Finished",
                            },
                        )
                        .size(18)
                    )
                    .center_x(Length::Fill),
                ],
                // row![
                //     container(text!("{}", self.pomodori.get_quote()).size(14),)
                //         .center(Length::Fill),
                // ],
                row![container(svg("assets/pomo_logo.svg").width(150)).center(Length::Fill)],
                row![
                    container(text!("{}", self.pomodori.get_quote()).size(14),)
                        .center_x(Length::Fill),
                ],
                row![
                    container(text!("{:0>2}:{:0>2}", minutes, seconds).size(25),)
                        .padding(4)
                        .style(container::bordered_box)
                        .center_x(Length::FillPortion(2)),
                    container(
                        text!(
                            "{}/{}",
                            self.pomodori.get_interval_count() + 1,
                            self.pomodori.get_interval_target()
                        )
                        .size(25),
                    )
                    .padding(4)
                    .style(container::bordered_box)
                    .center_x(Length::FillPortion(1))
                ]
                .spacing(4),
                row![progress_bar(0.0..=1.0, progress).girth(4)],
                row![
                    tooltip(
                        button(svg("assets/icons/restart.svg")).on_press_maybe(
                            match self.pomodori.get_state() {
                                pomodori::State::Ready => None,
                                _ => Some(Message::ResetTimer),
                            }
                        ),
                        container("Reset [R]")
                            .padding(4)
                            .style(container::rounded_box),
                        tooltip::Position::Top,
                    ),
                    tooltip(
                        button(match self.state {
                            State::Idle => svg("assets/icons/play.svg"),
                            State::Running { .. } => svg("assets/icons/pause.svg"),
                        })
                        .style(button::success)
                        .on_press(Message::ToggleTimer),
                        container(match self.state {
                            State::Idle => "Start [Space]",
                            State::Running { .. } => "Pause [Space]",
                        })
                        .padding(4)
                        .style(container::rounded_box),
                        tooltip::Position::Top,
                    ),
                    tooltip(
                        button(svg("assets/icons/skip-next.svg")).on_press_maybe(
                            match self.pomodori.get_state() {
                                pomodori::State::Finished => None,
                                _ => Some(Message::SkipInterval),
                            }
                        ),
                        container("Skip current interval [N]")
                            .padding(4)
                            .style(container::rounded_box),
                        tooltip::Position::Top,
                    ),
                    tooltip(
                        button(svg("assets/icons/settings.svg")).on_press(Message::Settings),
                        container("Settings [S]")
                            .padding(4)
                            .style(container::rounded_box),
                        tooltip::Position::Top,
                    ),
                ]
                .spacing(4)
            ]
            .spacing(4),
        )
        .padding(4)
        .center(Length::Fill)
        .into()
    }

    fn theme(&self) -> iced::Theme {
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
                keyboard::Key::Character("n") => Some(Message::SkipInterval),
                keyboard::Key::Character("s") => Some(Message::Settings),

                _ => None,
            }
        });

        Subscription::batch(vec![hotkeys, tick])
    }
}
