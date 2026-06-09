use iced::Color;
use material_colors::{color::Argb, scheme::Scheme as M3Scheme};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialScheme {
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub inverse_primary: Color,

    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,

    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,

    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,

    pub background: Color,
    pub on_background: Color,

    pub surface: Color,
    pub on_surface: Color,
    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub surface_tint: Color,

    pub outline: Color,
    pub outline_variant: Color,

    pub inverse_surface: Color,
    pub inverse_on_surface: Color,

    pub shadow: Color,
    pub scrim: Color,

    pub is_dark: bool,
}

impl MaterialScheme {
    pub(crate) fn from_m3(s: M3Scheme, is_dark: bool) -> Self {
        Self {
            primary: c(s.primary),
            on_primary: c(s.on_primary),
            primary_container: c(s.primary_container),
            on_primary_container: c(s.on_primary_container),
            inverse_primary: c(s.inverse_primary),

            secondary: c(s.secondary),
            on_secondary: c(s.on_secondary),
            secondary_container: c(s.secondary_container),
            on_secondary_container: c(s.on_secondary_container),

            tertiary: c(s.tertiary),
            on_tertiary: c(s.on_tertiary),
            tertiary_container: c(s.tertiary_container),
            on_tertiary_container: c(s.on_tertiary_container),

            error: c(s.error),
            on_error: c(s.on_error),
            error_container: c(s.error_container),
            on_error_container: c(s.on_error_container),

            background: c(s.background),
            on_background: c(s.on_background),

            surface: c(s.surface),
            on_surface: c(s.on_surface),
            surface_variant: c(s.surface_variant),
            on_surface_variant: c(s.on_surface_variant),
            surface_dim: c(s.surface_dim),
            surface_bright: c(s.surface_bright),
            surface_container_lowest: c(s.surface_container_lowest),
            surface_container_low: c(s.surface_container_low),
            surface_container: c(s.surface_container),
            surface_container_high: c(s.surface_container_high),
            surface_container_highest: c(s.surface_container_highest),
            surface_tint: c(s.surface_tint),

            outline: c(s.outline),
            outline_variant: c(s.outline_variant),

            inverse_surface: c(s.inverse_surface),
            inverse_on_surface: c(s.inverse_on_surface),

            shadow: c(s.shadow),
            scrim: c(s.scrim),

            is_dark,
        }
    }
}

fn c(argb: Argb) -> Color {
    Color::from_rgba8(argb.red, argb.green, argb.blue, argb.alpha as f32 / 255.0)
}

pub fn mix_alpha(base: Color, overlay: Color, alpha: f32) -> Color {
    let a = alpha.clamp(0.0, 1.0);
    Color::from_rgb(
        base.r * (1.0 - a) + overlay.r * a,
        base.g * (1.0 - a) + overlay.g * a,
        base.b * (1.0 - a) + overlay.b * a,
    )
}

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha.clamp(0.0, 1.0), ..color }
}
