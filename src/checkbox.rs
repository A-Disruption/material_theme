use iced::border::Radius;
use iced::widget::checkbox::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::scheme::{MaterialScheme, mix_alpha, with_alpha};
use crate::tokens::{disabled, state};
use crate::widget_common::scheme_for;

const RADIUS: f32 = 2.0;
const BORDER_WIDTH: f32 = 2.0;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);

    let (background, icon_color, border_color) = match status {
        Status::Active { is_checked: true } => {
            (Background::Color(s.primary), s.on_primary, s.primary)
        }
        Status::Active { is_checked: false } => (
            Background::Color(Color::TRANSPARENT),
            Color::TRANSPARENT,
            s.on_surface_variant,
        ),

        // M3 hover: an 8% state layer of the content color. Selected, that
        // layer sits on the primary container; unselected, the box is
        // transparent so the layer *is* the fill, and the outline darkens to
        // on-surface.
        Status::Hovered { is_checked: true } => (
            Background::Color(mix_alpha(s.primary, s.on_primary, state::HOVER)),
            s.on_primary,
            mix_alpha(s.primary, s.on_primary, state::HOVER),
        ),
        Status::Hovered { is_checked: false } => (
            Background::Color(with_alpha(s.on_surface, state::HOVER)),
            Color::TRANSPARENT,
            s.on_surface,
        ),

        Status::Disabled { is_checked } => return disabled_style(&s, is_checked),
    };

    Style {
        background,
        icon_color,
        border: Border {
            color: border_color,
            width: BORDER_WIDTH,
            radius: Radius::from(RADIUS),
        },
        text_color: Some(s.on_surface),
    }
}

fn disabled_style(s: &MaterialScheme, is_checked: bool) -> Style {
    let dim = with_alpha(s.on_surface, disabled::CONTENT);
    let (background, icon_color) = if is_checked {
        (Background::Color(dim), s.surface)
    } else {
        (Background::Color(Color::TRANSPARENT), Color::TRANSPARENT)
    };

    Style {
        background,
        icon_color,
        border: Border {
            color: dim,
            width: BORDER_WIDTH,
            radius: Radius::from(RADIUS),
        },
        text_color: Some(dim),
    }
}
