use iced::border::Radius;
use iced::widget::overlay::menu::Style;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

use crate::scheme::with_alpha;
use crate::widget_common::scheme_for;

const RADIUS: f32 = 4.0;

pub fn default(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        background: Background::Color(s.surface_container),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(RADIUS),
        },
        text_color: s.on_surface,
        selected_text_color: s.on_secondary_container,
        selected_background: Background::Color(s.secondary_container),
        shadow: Shadow {
            color: with_alpha(s.shadow, 0.30),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        },
    }
}
