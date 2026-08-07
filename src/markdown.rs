//! M3 styles for `markdown`.
//!
//! Requires the `markdown` feature, which enables `iced/markdown`.

use iced::widget::markdown::{Highlight, Settings, Style};
use iced::{Font, Theme, border, padding};

use crate::widget_common::scheme_for;

/// M3 markdown styling: links in the primary role, inline code on a tonal
/// container.
///
/// `iced`'s own default hardcodes near-black inline code, which is unreadable
/// on a dark surface; this follows the scheme in both modes.
pub fn style(theme: &Theme) -> Style {
    let s = scheme_for(theme);

    Style {
        font: Font::default(),
        inline_code_highlight: Highlight {
            background: s.surface_container_highest.into(),
            border: border::rounded(crate::tokens::shape::EXTRA_SMALL),
        },
        inline_code_padding: padding::left(4).right(4),
        inline_code_color: s.on_surface,
        inline_code_font: Font::MONOSPACE,
        code_block_font: Font::MONOSPACE,
        link_color: s.primary,
    }
}

/// [`Settings`] carrying [`style`], for passing to `markdown::view`.
pub fn settings(theme: &Theme) -> Settings {
    Settings::with_style(style(theme))
}

/// [`settings`] at a specific base text size.
pub fn settings_with_text_size(theme: &Theme, text_size: impl Into<iced::Pixels>) -> Settings {
    Settings::with_text_size(text_size, style(theme))
}
