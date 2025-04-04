use iced::{
    Color, Element, Length, Pixels, Size, Subscription, Task, Theme,
    font::{self, Family},
    theme::Palette,
    widget::container,
    window,
};

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

// #[derive(Default)]
pub struct Pomodorable {
    theme: Theme,
    currentSize: Size,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CloseWindow,
    WindowResized(iced::Size),
}

impl Pomodorable {
    fn new() -> (Pomodorable, Task<Message>) {
        let mouser = Pomodorable {
            theme: Theme::GruvboxDark,
            currentSize: Size::default(),
        };

        (mouser, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", message);

        match message {
            Message::CloseWindow => window::get_latest().and_then(window::close),
            Message::WindowResized(size) => {
                self.currentSize = size;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container("Hello, Pomodorable").center(Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::keyboard::on_key_press(|key, modifiers| match (key, modifiers) {
                (iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape), _) => {
                    Some(Message::CloseWindow)
                }
                _ => None,
            }),
            iced::window::resize_events().map(|(_id, size)| Message::WindowResized(size)),
        ])
    }
}
