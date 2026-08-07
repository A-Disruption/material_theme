use iced::border::Radius;
use iced::widget::container::Style;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

use crate::scheme::with_alpha;
use crate::tokens::shape;
use crate::widget_common::scheme_for;

const CARD_RADIUS: f32 = shape::MEDIUM;

pub fn filled_card(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        text_color: Some(s.on_surface),
        background: Some(Background::Color(s.surface_container_highest)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(CARD_RADIUS),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

pub fn elevated_card(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        text_color: Some(s.on_surface),
        background: Some(Background::Color(s.surface_container_low)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(CARD_RADIUS),
        },
        shadow: Shadow {
            color: with_alpha(s.shadow, 0.30),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        snap: false,
    }
}

pub fn outlined_card(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        text_color: Some(s.on_surface),
        background: Some(Background::Color(s.surface)),
        border: Border {
            color: s.outline_variant,
            width: 1.0,
            radius: Radius::from(CARD_RADIUS),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

pub fn surface(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        text_color: Some(s.on_surface),
        background: Some(Background::Color(s.surface)),
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

pub fn swatch_box(color: Color, on_color: Color) -> impl Fn(&Theme) -> Style {
    move |_theme: &Theme| Style {
        text_color: Some(on_color),
        background: Some(Background::Color(color)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}
