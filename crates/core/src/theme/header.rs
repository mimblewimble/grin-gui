use grin_gui_widgets::style::header::{StyleSheet, Appearance};
use iced::{Background, Color};
use super::Theme;

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
                text_color: Some(self.palette.normal.primary),
                background: Some(Background::Color(self.palette.base.foreground)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 10.0,
                offset_right: 25.0,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let appearance = self.appearance(style);

        match style {
            HeaderStyle::Default => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..self.palette.normal.primary
                })),
                ..appearance
            },
        }
    }
}