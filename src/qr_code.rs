//! M3 styles for `qr_code`.

use iced::Theme;
use iced::widget::qr_code::Style;

use crate::widget_common::scheme_for;

/// Scheme-aware QR code. Contrast is guaranteed because `on_surface` is
/// derived as the accessible pair for `surface`.
pub fn default(theme: &Theme) -> Style {
    let s = scheme_for(theme);

    Style {
        cell: s.on_surface,
        background: s.surface,
    }
}

/// Always black-on-white, regardless of mode.
///
/// Some scanners cope poorly with inverted (light-on-dark) codes, so prefer
/// this when the code must be scannable in a dark-mode UI.
pub fn scannable(_theme: &Theme) -> Style {
    Style {
        cell: iced::Color::BLACK,
        background: iced::Color::WHITE,
    }
}
