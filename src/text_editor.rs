//! M3 text field styles for `text_editor`, matching [`crate::text_input`].

use iced::border::Radius;
use iced::widget::text_editor::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::{MaterialScheme, with_alpha};
use crate::tokens::{disabled, shape};
use crate::widget_common::scheme_for;

fn filled_radius() -> Radius {
    Radius {
        top_left: shape::EXTRA_SMALL,
        top_right: shape::EXTRA_SMALL,
        bottom_left: 0.0,
        bottom_right: 0.0,
    }
}

/// Filled text field: tonal container with a bottom indicator.
pub fn filled(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (container, indicator, width) = match status {
        Status::Active => (s.surface_container_highest, s.on_surface_variant, 1.0),
        Status::Hovered => (s.surface_container_highest, s.on_surface, 1.0),
        Status::Focused { .. } => (s.surface_container_highest, s.primary, 2.0),
        Status::Disabled => (
            with_alpha(s.on_surface, disabled::SURFACE),
            with_alpha(s.on_surface, disabled::CONTENT),
            1.0,
        ),
    };

    let (value, placeholder) = content_colors(&s, &status);

    Style {
        background: Background::Color(container),
        border: Border {
            color: indicator,
            width,
            radius: filled_radius(),
        },
        placeholder,
        value,
        selection: with_alpha(s.primary, 0.3),
    }
}

/// Outlined text field: transparent with a full outline.
pub fn outlined(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (border_color, border_width) = match status {
        Status::Active => (s.outline, 1.0),
        Status::Hovered => (s.on_surface, 1.0),
        Status::Focused { .. } => (s.primary, 2.0),
        Status::Disabled => (with_alpha(s.on_surface, disabled::CONTENT), 1.0),
    };

    let (value, placeholder) = content_colors(&s, &status);

    Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: border_color,
            width: border_width,
            radius: Radius::from(shape::EXTRA_SMALL),
        },
        placeholder,
        value,
        selection: with_alpha(s.primary, 0.3),
    }
}

fn content_colors(s: &MaterialScheme, status: &Status) -> (Color, Color) {
    if matches!(status, Status::Disabled) {
        let dim = with_alpha(s.on_surface, disabled::CONTENT);
        (dim, dim)
    } else {
        (s.on_surface, s.on_surface_variant)
    }
}
