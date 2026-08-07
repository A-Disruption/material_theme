use iced::theme::palette::deviate;
use iced::{Color, Theme};

use crate::scheme::MaterialScheme;
use crate::theme::MaterialThemeExt;

pub fn scheme_for(theme: &Theme) -> MaterialScheme {
    theme.material_scheme().map(|s| *s).unwrap_or_else(|| fallback(theme))
}

/// How far past iced's background ramp the outline roles sit.
///
/// Every surface role below is a step of iced's background swatch, which spans
/// deviations of 0.03 (`weakest`) to 0.20 (`strongest`). Mapping the outlines
/// onto that same swatch makes them collide with whatever surface they happen
/// to be drawn on: an outlined button bordered in `background.strong` vanishes
/// on a card that is *also* `background.strong`, and a menu separator in
/// `background.weak` vanishes on a panel that is also `background.weak`.
///
/// Deviating past the end of the ramp keeps them on the same perceptual axis —
/// `deviate` darkens a light background and lightens a dark one, so this works
/// in both modes — while guaranteeing separation from every surface step.
///
/// These read a little stronger than M3's own outlines (T80/T50 in light). That
/// is deliberate: this is the fallback for themes that have no M3 scheme at
/// all, where a visible line matters more than an exact tone.
const OUTLINE_VARIANT_DEVIATION: f32 = 0.30;
const OUTLINE_DEVIATION: f32 = 0.50;

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

    // `deviate` always moves away from the base in whichever direction
    // contrasts, so on a light theme the far end of the ramp is the *dimmest*
    // surface and on a dark theme it is the brightest. Without this swap the
    // two roles come out inverted for every light theme.
    let (surface_dim, surface_bright) = if p.is_dark {
        (p.background.weaker.color, p.background.strongest.color)
    } else {
        (p.background.strongest.color, p.background.weaker.color)
    };

    MaterialScheme {
        primary,
        on_primary,
        primary_container: p.primary.weak.color,
        on_primary_container: p.primary.weak.text,
        inverse_primary: p.primary.strong.color,
        primary_fixed: p.primary.weak.color,
        primary_fixed_dim: p.primary.strong.color,
        on_primary_fixed: p.primary.weak.text,
        on_primary_fixed_variant: p.primary.weak.text,
        secondary,
        on_secondary,
        secondary_container: p.secondary.weak.color,
        on_secondary_container: p.secondary.weak.text,
        secondary_fixed: p.secondary.weak.color,
        secondary_fixed_dim: p.secondary.strong.color,
        on_secondary_fixed: p.secondary.weak.text,
        on_secondary_fixed_variant: p.secondary.weak.text,
        tertiary: primary,
        on_tertiary: on_primary,
        tertiary_container: p.primary.weak.color,
        on_tertiary_container: p.primary.weak.text,
        tertiary_fixed: p.primary.weak.color,
        tertiary_fixed_dim: p.primary.strong.color,
        on_tertiary_fixed: p.primary.weak.text,
        on_tertiary_fixed_variant: p.primary.weak.text,
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
        surface_dim,
        surface_bright,
        surface_container_lowest: p.background.weakest.color,
        surface_container_low: p.background.weaker.color,
        surface_container: p.background.weak.color,
        surface_container_high: p.background.neutral.color,
        surface_container_highest: p.background.strong.color,
        surface_tint: primary,
        outline: deviate(bg, OUTLINE_DEVIATION),
        outline_variant: deviate(bg, OUTLINE_VARIANT_DEVIATION),
        inverse_surface: on_bg,
        inverse_on_surface: bg,
        shadow: Color::BLACK,
        scrim: Color::BLACK,
        is_dark: p.is_dark,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn distance(a: Color, b: Color) -> f32 {
        ((a.r - b.r).powi(2) + (a.g - b.g).powi(2) + (a.b - b.b).powi(2)).sqrt()
    }

    /// An outline the same color as the surface it is drawn on is invisible,
    /// which is how `button::outlined` lost its border and `menu::separator`
    /// its rule on the built-in themes.
    #[test]
    fn outlines_are_visible_against_every_surface_on_built_in_themes() {
        for theme in iced::Theme::ALL {
            let s = scheme_for(theme);

            let surfaces = [
                ("surface", s.surface),
                ("surface_variant", s.surface_variant),
                ("surface_dim", s.surface_dim),
                ("surface_bright", s.surface_bright),
                ("surface_container_lowest", s.surface_container_lowest),
                ("surface_container_low", s.surface_container_low),
                ("surface_container", s.surface_container),
                ("surface_container_high", s.surface_container_high),
                ("surface_container_highest", s.surface_container_highest),
            ];

            for (role, outline) in [("outline", s.outline), ("outline_variant", s.outline_variant)]
            {
                for (name, surface) in surfaces {
                    assert!(
                        distance(outline, surface) > 0.05,
                        "{theme}: {role} is indistinguishable from {name}"
                    );
                }
            }
        }
    }

    /// The names have to mean what they say in both modes, which the raw
    /// background ramp does not give for free.
    #[test]
    fn surface_bright_is_brighter_than_surface_dim() {
        for theme in iced::Theme::ALL {
            let s = scheme_for(theme);

            assert!(
                s.surface_bright.relative_luminance() > s.surface_dim.relative_luminance(),
                "{theme}: surface_bright is darker than surface_dim"
            );
        }
    }

    /// `outline` is the load-bearing one — a button border, a focused field —
    /// so it has to read more strongly than the hairline `outline_variant`.
    #[test]
    fn outline_is_stronger_than_outline_variant() {
        for theme in iced::Theme::ALL {
            let s = scheme_for(theme);

            assert!(
                distance(s.outline, s.surface) > distance(s.outline_variant, s.surface),
                "{theme}: outline does not stand out more than outline_variant"
            );
        }
    }
}
