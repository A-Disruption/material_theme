//! M3 elevation for `float`.

use iced::Theme;
use iced::widget::float::Style;

use crate::tokens::{elevation, shape};
use crate::widget_common::scheme_for;

/// Builds a float style at the given M3 elevation level (0–5), with a corner
/// radius matching the content it lifts.
pub fn at_level(level: u8, radius: f32) -> impl Fn(&Theme) -> Style {
    move |theme: &Theme| Style {
        shadow: elevation(&scheme_for(theme), level),
        shadow_border_radius: radius.into(),
    }
}

/// Elevation level 3 with a medium (card) radius — the M3 default for a
/// lifted, draggable element.
pub fn default(theme: &Theme) -> Style {
    at_level(3, shape::MEDIUM)(theme)
}
