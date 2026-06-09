use iced::border::Radius;
use iced::widget::checkbox::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::with_alpha;
use crate::widget_common::scheme_for;

const RADIUS: f32 = 2.0;
const BORDER_WIDTH: f32 = 2.0;
const DISABLED_ALPHA: f32 = 0.38;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (is_checked, is_active) = match status {
        Status::Active { is_checked } => (is_checked, true),
        Status::Hovered { is_checked } => (is_checked, true),
        Status::Disabled { is_checked } => (is_checked, false),
    };

    let (bg, icon, border_color) = if !is_active {
        let dim = with_alpha(s.on_surface, DISABLED_ALPHA);
        if is_checked {
            (Background::Color(dim), s.surface, dim)
        } else {
            (Background::Color(Color::TRANSPARENT), Color::TRANSPARENT, dim)
        }
    } else if is_checked {
        (Background::Color(s.primary), s.on_primary, s.primary)
    } else {
        (Background::Color(Color::TRANSPARENT), Color::TRANSPARENT, s.on_surface_variant)
    };

    Style {
        background: bg,
        icon_color: icon,
        border: Border {
            color: border_color,
            width: BORDER_WIDTH,
            radius: Radius::from(RADIUS),
        },
        text_color: Some(if is_active { s.on_surface } else { with_alpha(s.on_surface, DISABLED_ALPHA) }),
    }
}
