//! M3 styles for `pick_list`.
//!
//! An M3 dropdown menu is a text field that opens a menu, so these mirror
//! [`crate::text_input::filled`] and [`crate::text_input::outlined`]: same
//! container, same indicator ramp, same shape. Pair either with
//! [`crate::menu::default`] for the list itself.

use iced::border::Radius;
use iced::widget::pick_list::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::{MaterialScheme, with_alpha};
use crate::tokens::{disabled, shape};
use crate::widget_common::scheme_for;

const RADIUS: f32 = shape::EXTRA_SMALL;

/// The M3 filled field is square along the bottom, where its indicator sits.
fn filled_radius() -> Radius {
    Radius {
        top_left: RADIUS,
        top_right: RADIUS,
        bottom_left: 0.0,
        bottom_right: 0.0,
    }
}

/// Label, placeholder and arrow colors, dimmed together when disabled.
fn content(s: &MaterialScheme, status: Status) -> (Color, Color, Color) {
    if matches!(status, Status::Disabled) {
        let dim = with_alpha(s.on_surface, disabled::CONTENT);

        (dim, dim, dim)
    } else {
        (s.on_surface, s.on_surface_variant, s.on_surface_variant)
    }
}

pub fn filled(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);

    let (background, indicator, border_width) = match status {
        Status::Active => (s.surface_container_highest, s.on_surface_variant, 1.0),
        Status::Hovered => (s.surface_container_highest, s.on_surface, 1.0),
        Status::Opened { .. } => (s.surface_container_highest, s.primary, 2.0),
        Status::Disabled => (
            with_alpha(s.on_surface, disabled::SURFACE),
            with_alpha(s.on_surface, disabled::CONTENT),
            1.0,
        ),
    };

    let (text_color, placeholder_color, handle_color) = content(&s, status);

    Style {
        text_color,
        placeholder_color,
        handle_color,
        background: Background::Color(background),
        border: Border {
            color: indicator,
            width: border_width,
            radius: filled_radius(),
        },
    }
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);

    let (border_color, border_width) = match status {
        Status::Active => (s.outline, 1.0),
        Status::Hovered => (s.on_surface, 1.0),
        Status::Opened { .. } => (s.primary, 2.0),

        // The outline is a container, not content: M3 dims it to 12%, not the
        // 38% the label inside it uses.
        Status::Disabled => (with_alpha(s.on_surface, disabled::CONTAINER), 1.0),
    };

    let (text_color, placeholder_color, handle_color) = content(&s, status);

    Style {
        text_color,
        placeholder_color,
        handle_color,
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: border_color,
            width: border_width,
            radius: Radius::from(RADIUS),
        },
    }
}
