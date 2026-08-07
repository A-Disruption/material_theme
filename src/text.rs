//! M3 role colors for `text`.
//!
//! `iced`'s default (`None`) inherits the surrounding color, which is usually
//! what you want. These are for pulling a specific role out of the scheme.

use iced::Theme;
use iced::widget::text::Style;

use crate::scheme::with_alpha;
use crate::tokens::disabled;
use crate::widget_common::scheme_for;

macro_rules! role {
    ($(#[$meta:meta])* $name:ident => $field:ident) => {
        $(#[$meta])*
        pub fn $name(theme: &Theme) -> Style {
            Style { color: Some(scheme_for(theme).$field) }
        }
    };
}

/// Inherits the surrounding text color.
pub fn default(_theme: &Theme) -> Style {
    Style { color: None }
}

role!(/// Body text on a surface.
    on_surface => on_surface);
role!(/// Supporting text, labels, and placeholders.
    on_surface_variant => on_surface_variant);
role!(/// Accented or emphasized text.
    primary => primary);
role!(/// Secondary accent text.
    secondary => secondary);
role!(/// Tertiary accent text.
    tertiary => tertiary);
role!(/// Error and validation text.
    error => error);
role!(/// Text on an inverted surface, e.g. a snackbar.
    inverse_on_surface => inverse_on_surface);

/// Body text at the M3 disabled opacity.
pub fn disabled(theme: &Theme) -> Style {
    Style {
        color: Some(with_alpha(scheme_for(theme).on_surface, disabled::CONTENT)),
    }
}
