//! M3 settings for `markdown`.
//!
//! Requires the `markdown` feature, which enables `iced/markdown`.
//!
//! `iced` moved markdown colors off [`Settings`] and onto the theme's
//! `markdown::Catalog`, whose impl for [`Theme`] follows the palette: links in
//! the seed's primary, inline code on the weaker background. That covers what
//! the old M3 `Style` override was for, so these are kept as thin wrappers for
//! callers that already pass through here.

use iced::Theme;
use iced::widget::markdown::Settings;

/// [`Settings`] for passing to `markdown::view`.
pub fn settings(_theme: &Theme) -> Settings {
    Settings::default()
}

/// [`settings`] at a specific base text size.
pub fn settings_with_text_size(_theme: &Theme, text_size: impl Into<iced::Pixels>) -> Settings {
    Settings::with_text_size(text_size)
}
