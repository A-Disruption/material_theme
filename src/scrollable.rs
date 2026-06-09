use iced::border::Radius;
use iced::widget::container;
use iced::widget::scrollable::{AutoScroll, Rail, Scroller, Status, Style};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

use crate::scheme::with_alpha;
use crate::widget_common::scheme_for;

const SCROLLER_RADIUS: f32 = 4.0;

pub fn default(theme: &Theme, status: Status) -> Style {
    let s = scheme_for(theme);
    let hovered = matches!(
        status,
        Status::Hovered { .. } | Status::Dragged { .. }
    );
    let scroller_color = if hovered {
        with_alpha(s.on_surface, 0.58)
    } else {
        with_alpha(s.on_surface, 0.38)
    };

    let rail = Rail {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border::default(),
        scroller: Scroller {
            background: Background::Color(scroller_color),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(SCROLLER_RADIUS),
            },
        },
    };

    Style {
        container: container::Style {
            text_color: Some(s.on_surface),
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        },
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: AutoScroll {
            background: Background::Color(s.surface_container_high),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(8.0),
            },
            shadow: Shadow {
                color: with_alpha(s.shadow, 0.30),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            icon: s.on_surface,
        },
    }
}
