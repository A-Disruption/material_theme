//! M3 styles for `pane_grid` drag/split affordances.

use iced::widget::pane_grid::{Highlight, Line, Style};
use iced::{Background, Border, Theme};

use crate::scheme::with_alpha;
use crate::tokens::state;
use crate::widget_common::scheme_for;

pub fn default(theme: &Theme) -> Style {
    let s = scheme_for(theme);

    Style {
        // A drop target reads as a tonal region, not a solid primary block.
        hovered_region: Highlight {
            background: Background::Color(with_alpha(s.primary, state::DRAGGED)),
            border: Border {
                color: s.primary,
                width: 2.0,
                radius: 0.0.into(),
            },
        },
        picked_split: Line {
            color: s.primary,
            width: 2.0,
        },
        hovered_split: Line {
            color: s.outline,
            width: 2.0,
        },
    }
}
