use super::Theme;
use grin_gui_widgets::style::header::{Appearance, StyleSheet};
use iced::{Background, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeaderStyle {
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = HeaderStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            HeaderStyle::Default => Appearance {
                text_color: None,
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_right: 0.0,
                offset_left: 0.0,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let appearance = self.appearance(style);
        Appearance {
            background: None,
            ..appearance
        }
    }
}
