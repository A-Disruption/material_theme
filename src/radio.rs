use iced::widget::radio::{Status, Style};
use iced::{Background, Color, Theme};

use crate::scheme::with_alpha;
use crate::widget_common::scheme_for;

const BORDER_WIDTH: f32 = 2.0;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (border_color, dot_color, text_alpha) = match status {
        Status::Active { is_selected: true } => (s.primary, s.primary, 1.0_f32),
        Status::Active { is_selected: false } => (s.on_surface_variant, Color::TRANSPARENT, 1.0),
        Status::Hovered { is_selected: true } => (s.primary, s.primary, 1.0),
        Status::Hovered { is_selected: false } => (s.on_surface, Color::TRANSPARENT, 1.0),
    };
    Style {
        background: Background::Color(Color::TRANSPARENT),
        dot_color,
        border_width: BORDER_WIDTH,
        border_color,
        text_color: Some(if text_alpha < 1.0 {
            with_alpha(s.on_surface, text_alpha)
        } else {
            s.on_surface
        }),
    }
}
