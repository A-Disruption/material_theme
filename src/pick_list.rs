use iced::border::Radius;
use iced::widget::pick_list::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::widget_common::scheme_for;

const RADIUS: f32 = 4.0;

pub fn filled(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (bg, border_color, border_width) = match status {
        Status::Active => (s.surface_container_highest, s.outline, 1.0),
        Status::Hovered => (s.surface_container_highest, s.on_surface, 1.0),
        Status::Opened { .. } => (s.surface_container_highest, s.primary, 2.0),
        Status::Disabled => (s.surface_container, s.outline_variant, 1.0),
    };
    Style {
        text_color: s.on_surface,
        placeholder_color: s.on_surface_variant,
        handle_color: s.on_surface_variant,
        background: Background::Color(bg),
        border: Border {
            color: border_color,
            width: border_width,
            radius: Radius::from(RADIUS),
        },
    }
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (border_color, border_width) = match status {
        Status::Active => (s.outline, 1.0),
        Status::Hovered => (s.on_surface, 1.0),
        Status::Opened { .. } => (s.primary, 2.0),
        Status::Disabled => (s.outline_variant, 1.0),
    };
    Style {
        text_color: s.on_surface,
        placeholder_color: s.on_surface_variant,
        handle_color: s.on_surface_variant,
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: border_color,
            width: border_width,
            radius: Radius::from(RADIUS),
        },
    }
}
