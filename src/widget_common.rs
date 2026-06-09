use iced::{Color, Theme};

use crate::scheme::MaterialScheme;
use crate::theme::MaterialThemeExt;

pub fn scheme_for(theme: &Theme) -> MaterialScheme {
    theme.material_scheme().map(|s| *s).unwrap_or_else(|| fallback(theme))
}

fn fallback(theme: &Theme) -> MaterialScheme {
    let p = theme.palette();
    let bg = p.background.base.color;
    let on_bg = p.background.base.text;
    let primary = p.primary.base.color;
    let on_primary = p.primary.base.text;
    let secondary = p.secondary.base.color;
    let on_secondary = p.secondary.base.text;
    let danger = p.danger.base.color;
    let on_danger = p.danger.base.text;
    MaterialScheme {
        primary,
        on_primary,
        primary_container: p.primary.weak.color,
        on_primary_container: p.primary.weak.text,
        inverse_primary: p.primary.strong.color,
        secondary,
        on_secondary,
        secondary_container: p.secondary.weak.color,
        on_secondary_container: p.secondary.weak.text,
        tertiary: primary,
        on_tertiary: on_primary,
        tertiary_container: p.primary.weak.color,
        on_tertiary_container: p.primary.weak.text,
        error: danger,
        on_error: on_danger,
        error_container: p.danger.weak.color,
        on_error_container: p.danger.weak.text,
        background: bg,
        on_background: on_bg,
        surface: bg,
        on_surface: on_bg,
        surface_variant: p.background.weak.color,
        on_surface_variant: p.background.weak.text,
        surface_dim: p.background.weaker.color,
        surface_bright: p.background.strongest.color,
        surface_container_lowest: p.background.weakest.color,
        surface_container_low: p.background.weaker.color,
        surface_container: p.background.weak.color,
        surface_container_high: p.background.neutral.color,
        surface_container_highest: p.background.strong.color,
        surface_tint: primary,
        outline: p.background.strong.color,
        outline_variant: p.background.weak.color,
        inverse_surface: on_bg,
        inverse_on_surface: bg,
        shadow: Color::BLACK,
        scrim: Color::BLACK,
        is_dark: p.is_dark,
    }
}
