//! M3 styles for `slider`.
//!
//! These follow the pre-2024 M3 slider: a 4dp track with a 20dp circular
//! handle. The December 2023 redesign — a 16dp track with a 4dp bar handle —
//! also calls for a gap between the active and inactive track, a stop
//! indicator at the track end and a rounded-rectangle value indicator, none of
//! which [`iced::widget::slider::Style`] can express. Rendering half of that
//! design reads worse than rendering all of the previous one.

use iced::border::Radius;
use iced::widget::slider::{Handle, HandleShape, Rail, Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::with_alpha;
use crate::tokens::disabled;
use crate::widget_common::scheme_for;

/// M3 slider track height.
const TRACK_WIDTH: f32 = 4.0;

/// Half the 20dp M3 handle.
const HANDLE_RADIUS: f32 = 10.0;

/// The handle grows under the cursor, standing in for the M3 state layer that
/// `iced` gives the slider no separate slot for.
const HANDLE_RADIUS_ACTIVE: f32 = HANDLE_RADIUS + 2.0;

const DISABLED_CONTENT_ALPHA: f32 = disabled::CONTENT;
const DISABLED_TRACK_ALPHA: f32 = disabled::CONTAINER;

fn rail(active: Color, inactive: Color) -> Rail {
    Rail {
        backgrounds: (Background::Color(active), Background::Color(inactive)),
        width: TRACK_WIDTH,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(TRACK_WIDTH / 2.0),
        },
    }
}

fn handle(color: Color, radius: f32) -> Handle {
    Handle {
        shape: HandleShape::Circle { radius },
        background: Background::Color(color),
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);

    let radius = match status {
        Status::Hovered | Status::Dragged => HANDLE_RADIUS_ACTIVE,
        Status::Active => HANDLE_RADIUS,
    };

    Style {
        rail: rail(s.primary, s.surface_container_highest),
        handle: handle(s.primary, radius),
    }
}

pub fn disabled(theme: &Theme, _status: Status) -> Style {
    let s = scheme_for(theme);

    let dim = with_alpha(s.on_surface, DISABLED_CONTENT_ALPHA);
    let dim_track = with_alpha(s.on_surface, DISABLED_TRACK_ALPHA);

    Style {
        rail: rail(dim, dim_track),
        handle: handle(dim, HANDLE_RADIUS),
    }
}
