use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

use crate::scheme::{MaterialScheme, mix_alpha, with_alpha};
use crate::tokens::{disabled, shape, state};
use crate::widget_common::scheme_for as scheme;

const M3_BUTTON_RADIUS: f32 = shape::FULL;
const HOVER_STATE_LAYER: f32 = state::HOVER;
const PRESSED_STATE_LAYER: f32 = state::PRESSED;
const DISABLED_CONTAINER_ALPHA: f32 = disabled::CONTAINER;
const DISABLED_CONTENT_ALPHA: f32 = disabled::CONTENT;

fn rounded(bg: Option<Background>, text: Color) -> Style {
    Style {
        background: bg,
        text_color: text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: M3_BUTTON_RADIUS.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn disabled_filled(s: &MaterialScheme) -> Style {
    rounded(
        Some(Background::Color(with_alpha(s.on_surface, DISABLED_CONTAINER_ALPHA))),
        with_alpha(s.on_surface, DISABLED_CONTENT_ALPHA),
    )
}

fn disabled_text(s: &MaterialScheme) -> Style {
    rounded(None, with_alpha(s.on_surface, DISABLED_CONTENT_ALPHA))
}

pub fn filled(theme: &Theme, status: Status) -> Style {
    let s = scheme(theme);
    match status {
        Status::Active => rounded(Some(Background::Color(s.primary)), s.on_primary),
        Status::Hovered => rounded(
            Some(Background::Color(mix_alpha(s.primary, s.on_primary, HOVER_STATE_LAYER))),
            s.on_primary,
        ),
        Status::Pressed => rounded(
            Some(Background::Color(mix_alpha(s.primary, s.on_primary, PRESSED_STATE_LAYER))),
            s.on_primary,
        ),
        Status::Disabled => disabled_filled(&s),
    }
}

pub fn tonal(theme: &Theme, status: Status) -> Style {
    let s = scheme(theme);
    let bg = s.secondary_container;
    let fg = s.on_secondary_container;
    match status {
        Status::Active => rounded(Some(Background::Color(bg)), fg),
        Status::Hovered => rounded(
            Some(Background::Color(mix_alpha(bg, fg, HOVER_STATE_LAYER))),
            fg,
        ),
        Status::Pressed => rounded(
            Some(Background::Color(mix_alpha(bg, fg, PRESSED_STATE_LAYER))),
            fg,
        ),
        Status::Disabled => disabled_filled(&s),
    }
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let s = scheme(theme);
    let border = |color: Color| Border {
        color,
        width: 1.0,
        radius: M3_BUTTON_RADIUS.into(),
    };
    match status {
        Status::Active => Style {
            background: None,
            text_color: s.primary,
            border: border(s.outline),
            shadow: Shadow::default(),
            snap: false,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(with_alpha(s.primary, HOVER_STATE_LAYER))),
            text_color: s.primary,
            border: border(s.outline),
            shadow: Shadow::default(),
            snap: false,
        },
        Status::Pressed => Style {
            background: Some(Background::Color(with_alpha(s.primary, PRESSED_STATE_LAYER))),
            text_color: s.primary,
            border: border(s.primary),
            shadow: Shadow::default(),
            snap: false,
        },
        Status::Disabled => Style {
            background: None,
            text_color: with_alpha(s.on_surface, DISABLED_CONTENT_ALPHA),
            border: border(with_alpha(s.on_surface, DISABLED_CONTAINER_ALPHA)),
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

pub fn elevated(theme: &Theme, status: Status) -> Style {
    let s = scheme(theme);
    let bg = s.surface_container_low;
    let fg = s.primary;
    let shadow = Shadow {
        color: with_alpha(s.shadow, 0.30),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 3.0,
    };
    match status {
        Status::Active => Style {
            background: Some(Background::Color(bg)),
            text_color: fg,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: M3_BUTTON_RADIUS.into(),
            },
            shadow,
            snap: false,
        },
        Status::Hovered => Style {
            background: Some(Background::Color(mix_alpha(bg, fg, HOVER_STATE_LAYER))),
            text_color: fg,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: M3_BUTTON_RADIUS.into(),
            },
            shadow: Shadow {
                color: with_alpha(s.shadow, 0.30),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            snap: false,
        },
        Status::Pressed => Style {
            background: Some(Background::Color(mix_alpha(bg, fg, PRESSED_STATE_LAYER))),
            text_color: fg,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: M3_BUTTON_RADIUS.into(),
            },
            shadow,
            snap: false,
        },
        Status::Disabled => disabled_filled(&s),
    }
}

pub fn text(theme: &Theme, status: Status) -> Style {
    let s = scheme(theme);
    let fg = s.primary;
    match status {
        Status::Active => rounded(None, fg),
        Status::Hovered => rounded(Some(Background::Color(with_alpha(fg, HOVER_STATE_LAYER))), fg),
        Status::Pressed => rounded(Some(Background::Color(with_alpha(fg, PRESSED_STATE_LAYER))), fg),
        Status::Disabled => disabled_text(&s),
    }
}
