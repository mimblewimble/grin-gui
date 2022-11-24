use iced::widget::container;
use iced::{Background, Color};

use super::ColorPalette;
use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    BrightForeground,
    BrightBackground,
    ErrorForeground,
    NormalBackground,
    HoverableForeground,
    HoverableBrightForeground,
    SuccessBackground,
    Segmented,
    PanelBordered,
    PanelForeground,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::BrightBackground => container::Appearance {
                background: Some(Background::Color(self.palette.base.background)),
                text_color: Some(self.palette.bright.surface),
                ..container::Appearance::default()
            },
            Container::BrightForeground => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.bright.surface),
                ..container::Appearance::default()
            },
            Container::ErrorForeground => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::NormalBackground => container::Appearance {
                background: Some(Background::Color(self.palette.base.background)),
                text_color: Some(self.palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::Segmented => container::Appearance {
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
                ..container::Appearance::default()
            },
            Container::HoverableForeground => container::Appearance {
                background: None,
                text_color: Some(self.palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::HoverableBrightForeground => container::Appearance {
                background: None,
                text_color: Some(self.palette.bright.primary),
                ..container::Appearance::default()
            },
            Container::SuccessBackground => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::PanelForeground => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.bright.primary),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Container::PanelBordered => container::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Some(self.palette.bright.primary),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
            },
        }
    }
}
