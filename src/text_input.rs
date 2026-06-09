use iced::border::Radius;
use iced::widget::text_input::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::{MaterialScheme, with_alpha};
use crate::widget_common::scheme_for;

const FILLED_RADIUS_TOP: f32 = 4.0;
const OUTLINED_RADIUS: f32 = 4.0;
const DISABLED_ALPHA: f32 = 0.38;
const DISABLED_CONTAINER_ALPHA: f32 = 0.04;

fn filled_radius() -> Radius {
    Radius {
        top_left: FILLED_RADIUS_TOP,
        top_right: FILLED_RADIUS_TOP,
        bottom_left: 0.0,
        bottom_right: 0.0,
    }
}

pub fn filled(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (container, indicator) = match status {
        Status::Active => (s.surface_container_highest, s.on_surface_variant),
        Status::Hovered => (s.surface_container_highest, s.on_surface),
        Status::Focused { .. } => (s.surface_container_highest, s.primary),
        Status::Disabled => (
            with_alpha(s.on_surface, DISABLED_CONTAINER_ALPHA),
            with_alpha(s.on_surface, DISABLED_ALPHA),
        ),
    };
    let (text_color, placeholder) = disabled_text_colors(&s, &status);
    Style {
        background: Background::Color(container),
        border: Border {
            color: indicator,
            width: focus_width(&status),
            radius: filled_radius(),
        },
        icon: text_color,
        placeholder,
        value: text_color,
        selection: with_alpha(s.primary, 0.3),
    }
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (border_color, border_width) = match status {
        Status::Active => (s.outline, 1.0),
        Status::Hovered => (s.on_surface, 1.0),
        Status::Focused { .. } => (s.primary, 2.0),
        Status::Disabled => (with_alpha(s.on_surface, DISABLED_ALPHA), 1.0),
    };
    let (text_color, placeholder) = disabled_text_colors(&s, &status);
    Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: border_color,
            width: border_width,
            radius: Radius::from(OUTLINED_RADIUS),
        },
        icon: text_color,
        placeholder,
        value: text_color,
        selection: with_alpha(s.primary, 0.3),
    }
}

fn focus_width(status: &Status) -> f32 {
    if matches!(status, Status::Focused { .. }) { 2.0 } else { 1.0 }
}

fn disabled_text_colors(s: &MaterialScheme, status: &Status) -> (Color, Color) {
    if matches!(status, Status::Disabled) {
        (with_alpha(s.on_surface, DISABLED_ALPHA), with_alpha(s.on_surface, DISABLED_ALPHA))
    } else {
        (s.on_surface, s.on_surface_variant)
    }
}
