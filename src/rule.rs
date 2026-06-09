use iced::border::Radius;
use iced::widget::rule::{FillMode, Style};
use iced::Theme;

use crate::widget_common::scheme_for;

pub fn divider(theme: &Theme) -> Style {
    let s = scheme_for(theme);
    Style {
        color: s.outline_variant,
        radius: Radius::from(0.0),
        fill_mode: FillMode::Full,
        snap: false,
    }
}
