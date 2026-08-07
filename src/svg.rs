//! M3 icon tinting for `svg`.
//!
//! These apply a color filter, so they only make sense for symbolic icons.
//! Use [`original`] to leave a multi-color asset alone.

use iced::Theme;
use iced::widget::svg::{Status, Style};

use crate::scheme::with_alpha;
use crate::tokens::state;
use crate::widget_common::scheme_for;

/// Keeps the asset's own colors.
pub fn original(_theme: &Theme, _status: Status) -> Style {
    Style { color: None }
}

/// Standard icon color, brightening toward on-surface on hover.
pub fn on_surface_variant(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    Style {
        color: Some(match status {
            Status::Idle => s.on_surface_variant,
            Status::Hovered => s.on_surface,
        }),
    }
}

/// Accented icon, with an M3 hover state layer folded into the tint.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    Style {
        color: Some(match status {
            Status::Idle => s.primary,
            Status::Hovered => with_alpha(s.primary, 1.0 - state::HOVER),
        }),
    }
}

/// Error/destructive icon.
pub fn error(theme: &Theme, _status: Status) -> Style {
    Style {
        color: Some(scheme_for(theme).error),
    }
}
