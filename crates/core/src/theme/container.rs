use iced::widget::container;
use iced::{Background, Color};

use super::ColorPalette;
use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    BrightForeground(ColorPalette),
    BrightBackground(ColorPalette),
    ErrorForeground(ColorPalette),
    NormalBackground(ColorPalette),
    HoverableForeground(ColorPalette),
    HoverableBrightForeground(ColorPalette),
    SuccessBackground(ColorPalette),
    Segmented(ColorPalette),
    PanelBordered(ColorPalette), 
    PanelForeground(ColorPalette),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::BrightBackground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::BrightForeground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.bright.surface),
                ..container::Appearance::default()
            },
            Container::ErrorForeground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::NormalBackground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::Segmented(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::HoverableForeground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::HoverableBrightForeground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::SuccessBackground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.normal.surface),
                ..container::Appearance::default()
            },
            Container::PanelForeground(palette) => container::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: Some(palette.bright.primary),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..palette.normal.primary
                },
            },
            Container::PanelBordered(palette) => container::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Some(palette.bright.primary),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..palette.normal.primary
                },
            },
        }
    }
}
