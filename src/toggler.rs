use iced::widget::toggler::{Status, Style};
use iced::{Background, Color, Theme};

use crate::scheme::with_alpha;
use crate::widget_common::scheme_for;

const DISABLED_TRACK_ALPHA: f32 = 0.12;
const DISABLED_THUMB_ALPHA: f32 = 0.38;
const BORDER_WIDTH: f32 = 2.0;

pub fn switch(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (track, thumb, border, text_alpha) = match status {
        Status::Active { is_toggled: true } | Status::Hovered { is_toggled: true } => {
            (s.primary, s.on_primary, s.primary, 1.0_f32)
        }
        Status::Active { is_toggled: false } | Status::Hovered { is_toggled: false } => {
            (s.surface_container_highest, s.outline, s.outline, 1.0)
        }
        Status::Disabled { is_toggled: true } => (
            with_alpha(s.on_surface, DISABLED_TRACK_ALPHA),
            s.surface,
            Color::TRANSPARENT,
            DISABLED_THUMB_ALPHA,
        ),
        Status::Disabled { is_toggled: false } => (
            with_alpha(s.surface_container_highest, DISABLED_TRACK_ALPHA),
            with_alpha(s.on_surface, DISABLED_THUMB_ALPHA),
            with_alpha(s.on_surface, DISABLED_TRACK_ALPHA),
            DISABLED_THUMB_ALPHA,
        ),
    };

    Style {
        background: Background::Color(track),
        background_border_width: BORDER_WIDTH,
        background_border_color: border,
        foreground: Background::Color(thumb),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(if text_alpha < 1.0 {
            with_alpha(s.on_surface, text_alpha)
        } else {
            s.on_surface
        }),
        border_radius: None,
        padding_ratio: 0.15,
    }
}
