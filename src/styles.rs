use iced::{Vector, border, Color, Shadow, Theme, Background};
use iced::widget::{button};

// Modify Theme instead of setting color in Text Widgets, etc. 
pub fn clear_button(theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::from_rgb8(170, 170, 170))),
        border: border::rounded(10),
        ..button::Style::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb8(130, 130, 130))),
            ..base
        },
        button::Status::Disabled => disabled(base),
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb8(130, 130, 130))),
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(1.3, 1.3),
                blur_radius: 1.0,
            },
            ..base
        },
    }
}

fn disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}