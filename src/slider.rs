use iced::border::Radius;
use iced::widget::slider::{Handle, HandleShape, Rail, Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::with_alpha;
use crate::widget_common::scheme_for;

const TRACK_WIDTH: f32 = 16.0;
const HANDLE_RADIUS: f32 = 10.0;
const DISABLED_ALPHA: f32 = 0.38;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let (active, inactive, handle_color) = (s.primary, s.secondary_container, s.primary);
    let handle_radius = match status {
        Status::Hovered | Status::Dragged => HANDLE_RADIUS + 2.0,
        Status::Active => HANDLE_RADIUS,
    };

    Style {
        rail: Rail {
            backgrounds: (
                Background::Color(active),
                Background::Color(inactive),
            ),
            width: TRACK_WIDTH,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(TRACK_WIDTH / 2.0),
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: handle_radius },
            background: Background::Color(handle_color),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

pub fn disabled(theme: &Theme, _status: Status) -> Style {
    let s = scheme_for(theme);
    let dim = with_alpha(s.on_surface, DISABLED_ALPHA);
    let dim_track = with_alpha(s.on_surface, 0.12);
    Style {
        rail: Rail {
            backgrounds: (Background::Color(dim), Background::Color(dim_track)),
            width: TRACK_WIDTH,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(TRACK_WIDTH / 2.0),
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: HANDLE_RADIUS },
            background: Background::Color(dim),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}
