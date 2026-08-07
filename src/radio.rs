use iced::widget::radio::{Status, Style};
use iced::{Background, Color, Theme};

use crate::scheme::with_alpha;
use crate::tokens::state;
use crate::widget_common::scheme_for;

const BORDER_WIDTH: f32 = 2.0;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);

    // A radio has no fill of its own, so the M3 8% hover state layer is drawn
    // as the background: primary when selected, on-surface when not. The
    // unselected outline also darkens from on-surface-variant to on-surface.
    let (background, dot_color, border_color) = match status {
        Status::Active { is_selected: true } => {
            (Color::TRANSPARENT, s.primary, s.primary)
        }
        Status::Active { is_selected: false } => {
            (Color::TRANSPARENT, Color::TRANSPARENT, s.on_surface_variant)
        }
        Status::Hovered { is_selected: true } => (
            with_alpha(s.primary, state::HOVER),
            s.primary,
            s.primary,
        ),
        Status::Hovered { is_selected: false } => (
            with_alpha(s.on_surface, state::HOVER),
            Color::TRANSPARENT,
            s.on_surface,
        ),
    };

    Style {
        background: Background::Color(background),
        dot_color,
        border_width: BORDER_WIDTH,
        border_color,
        text_color: Some(s.on_surface),
    }
}
