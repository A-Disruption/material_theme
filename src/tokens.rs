//! Material Design 3 design tokens shared by every style function in this
//! crate, and by any third-party widget that wants to match.
//!
//! Keeping these here means a downstream widget can render a hover state that
//! is pixel-identical to the built-in widgets rather than guessing at 8%.

use iced::{Shadow, Vector};

use crate::scheme::{MaterialScheme, with_alpha};

/// State layer opacities.
///
/// M3 composites a translucent layer of the *content* color over a component
/// to signal interaction. Use with [`crate::mix_alpha`] when the component has
/// a background, and [`crate::with_alpha`] when it is transparent.
pub mod state {
    /// Opacity of the hover state layer (M3: 8%).
    pub const HOVER: f32 = 0.08;
    /// Opacity of the focus state layer (M3: 10%).
    pub const FOCUS: f32 = 0.10;
    /// Opacity of the pressed state layer (M3: 10%).
    pub const PRESSED: f32 = 0.10;
    /// Opacity of the dragged state layer (M3: 16%).
    pub const DRAGGED: f32 = 0.16;
}

/// Opacities applied to disabled components.
pub mod disabled {
    /// Opacity of disabled text, icons, and outlines (M3: 38%).
    pub const CONTENT: f32 = 0.38;
    /// Opacity of a disabled component's container fill (M3: 12%).
    pub const CONTAINER: f32 = 0.12;
    /// Opacity of a disabled *filled* container, e.g. a text field (M3: 4%).
    pub const SURFACE: f32 = 0.04;
}

/// The M3 shape scale, in logical pixels.
pub mod shape {
    /// Square corners.
    pub const NONE: f32 = 0.0;
    /// Extra small — text fields, menus, snackbars.
    pub const EXTRA_SMALL: f32 = 4.0;
    /// Small — chips.
    pub const SMALL: f32 = 8.0;
    /// Medium — cards.
    pub const MEDIUM: f32 = 12.0;
    /// Large — navigation drawers, bottom sheets.
    pub const LARGE: f32 = 16.0;
    /// Extra large — dialogs, large FABs.
    pub const EXTRA_LARGE: f32 = 28.0;
    /// Fully rounded — buttons, switches, sliders.
    ///
    /// M3 specifies "full" as half the component height. 20.0 matches the
    /// standard 40dp button.
    pub const FULL: f32 = 20.0;
}

/// The M3 elevation scale, expressed as an iced [`Shadow`].
///
/// Levels above 5 are clamped. Level 0 produces no shadow.
pub fn elevation(scheme: &MaterialScheme, level: u8) -> Shadow {
    // M3 elevation is a two-part shadow; iced renders one, so these are the
    // key-light values with the ambient contribution folded into the alpha.
    let (y_offset, blur, alpha) = match level.min(5) {
        0 => return Shadow::default(),
        1 => (1.0, 3.0, 0.30),
        2 => (2.0, 6.0, 0.30),
        3 => (4.0, 8.0, 0.30),
        4 => (6.0, 10.0, 0.30),
        _ => (8.0, 12.0, 0.30),
    };

    Shadow {
        color: with_alpha(scheme.shadow, alpha),
        offset: Vector::new(0.0, y_offset),
        blur_radius: blur,
    }
}
