use iced::Color;
use iced::theme::palette::Seed;

use crate::scheme::MaterialScheme;

pub fn to_iced_seed(scheme: &MaterialScheme) -> Seed {
    Seed {
        background: scheme.surface,
        text: scheme.on_surface,
        primary: scheme.primary,
        success: if scheme.is_dark {
            Color::from_rgb8(0x81, 0xC9, 0x95)
        } else {
            Color::from_rgb8(0x2E, 0x7D, 0x32)
        },
        warning: if scheme.is_dark {
            Color::from_rgb8(0xFF, 0xB8, 0x3D)
        } else {
            Color::from_rgb8(0xC7, 0x71, 0x00)
        },
        danger: scheme.error,
    }
}
