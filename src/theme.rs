use iced::{Color, Theme, theme::Palette};

pub struct Everforest;

impl Everforest {
    pub fn light_medium() -> Theme {
        Theme::custom(
            "Everforest Light Medium".to_string(),
            Palette {
                background: Color::from_rgb8(253, 246, 227),
                text: Color::from_rgb8(92, 106, 114),
                primary: Color::from_rgb8(230, 104, 104),
                success: Color::from_rgb8(147, 178, 89),
                warning: Color::from_rgb8(223, 160, 0),
                danger: Color::from_rgb8(248, 85, 82),
            },
        )
    }
    pub fn dark_medium() -> Theme {
        Theme::custom(
            "Everforest Light Medium".to_string(),
            Palette {
                background: Color::from_rgb8(45, 53, 59),
                text: Color::from_rgb8(211, 198, 170),
                primary: Color::from_rgb8(230, 126, 128),
                success: Color::from_rgb8(167, 192, 128),
                warning: Color::from_rgb8(219, 188, 127),
                danger: Color::from_rgb8(230, 126, 128),
            },
        )
    }
}
