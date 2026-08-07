use iced::border::Radius;
use iced::widget::progress_bar::Style;
use iced::{Background, Border, Color, Theme};

use crate::widget_common::scheme_for;

/// M3 linear progress indicator height.
const HEIGHT: f32 = 4.0;

pub fn primary(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        background: Background::Color(s.surface_container_highest),
        bar: Background::Color(s.primary),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(HEIGHT / 2.0),
        },
    }
}
