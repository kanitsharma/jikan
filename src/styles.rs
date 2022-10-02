use iced::{container, text_input, button};
use iced::{Background, Color};

pub enum Container {
    Default,
}

pub enum Input {
    Default,
}

pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgba8(227, 66, 50, 1.0),
                Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
            })),
            border_radius: 6.0,
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            ..self.active()
        }
    }
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

impl text_input::StyleSheet for Input {
    fn active(&self) -> iced::text_input::Style {
        text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border_width: 0.5,
            border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
            border_radius: 6.0,
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> iced::text_input::Style {
        text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border_width: 0.5,
            border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
            border_radius: 6.0,
            ..text_input::Style::default()
        }
    }

    fn hovered(&self) -> iced::text_input::Style {
        text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border_width: 0.5,
            border_color: Color::from_rgba8(0xEE, 0xEE, 0xEE, 0.5),
            border_radius: 6.0,
            ..text_input::Style::default()
        }
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb8(0xEE, 0xEE, 0xEE)
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb8(0xEE, 0xEE, 0xEE)
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }
}