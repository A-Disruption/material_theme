//! M3 styles for `table`.

use iced::Theme;
use iced::widget::table::Style;

use crate::widget_common::scheme_for;

/// Separators drawn in the M3 `outline_variant` role, per the divider spec.
pub fn default(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    let separator = s.outline_variant.into();

    Style {
        separator_x: separator,
        separator_y: separator,
    }
}

/// Row separators only — the M3 default for data tables.
pub fn horizontal(theme: &Theme) -> Style {
    let s = scheme_for(theme);

    Style {
        separator_x: s.outline_variant.into(),
        separator_y: iced::Color::TRANSPARENT.into(),
    }
}
