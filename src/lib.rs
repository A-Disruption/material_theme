//! Material Design 3 theming for [`iced`].
//!
//! A [`MaterialTheme`] converts into an [`iced::Theme`]; every module here is a
//! set of style functions for one built-in widget:
//!
//! ```ignore
//! use material_theme::{MaterialTheme, Mode, button as m3_button};
//!
//! fn theme(state: &State) -> iced::Theme {
//!     MaterialTheme::from_seed(0xFF6750A4, Mode::Dark).into()
//! }
//!
//! button("Save").style(m3_button::filled)
//! ```
//!
//! To style a widget this crate doesn't cover — including third-party widgets —
//! use [`scheme_for`] to pull the [`MaterialScheme`] out of the active theme,
//! and [`tokens`] for the M3 state-layer opacities and shape scale.

pub mod button;
pub mod checkbox;
pub mod container;
pub mod float;
pub mod menu;
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod rule;
pub mod scheme;
pub mod scrollable;
mod seed_map;
pub mod slider;
pub mod table;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod theme;
pub mod toggler;
pub mod tokens;
mod widget_common;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "qr_code")]
pub mod qr_code;

#[cfg(feature = "markdown")]
pub mod markdown;

pub use scheme::{MaterialScheme, mix_alpha, with_alpha};
pub use theme::{MaterialTheme, MaterialThemeExt, Mode, Variant};

/// Resolves the M3 scheme behind a theme, falling back to an approximation
/// derived from iced's own palette for non-Material themes.
///
/// This is what every style function in this crate is built on; it is exported
/// so downstream crates can style their own widgets the same way:
///
/// ```ignore
/// pub fn material(theme: &iced::Theme, status: Status) -> Style {
///     let s = material_theme::scheme_for(theme);
///     Style { background: s.surface_container_high, ..Default::default() }
/// }
/// ```
pub use widget_common::scheme_for;
