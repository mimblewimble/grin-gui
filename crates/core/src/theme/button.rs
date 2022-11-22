use iced::widget::button;
use iced::{Background, Color};

use super::ColorPalette;
use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Default,
    Bordered(ColorPalette),
    ColumnHeader(ColorPalette),
    Primary(ColorPalette),
    Selected(ColorPalette),
    SelectedColumn(ColorPalette),
    NormalText(ColorPalette),
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance::default(),
            Button::Bordered(palette) => button::Appearance {
                border_color: Color {
                    a: 0.5,
                    ..palette.normal.primary
                },
                border_width: 1.0,
                border_radius: 2.0,
                text_color: palette.bright.primary,
                ..button::Appearance::default()
            },
            Button::Primary(palette) => button::Appearance {
                text_color: palette.bright.primary,
                border_radius: 2.0,
                ..Default::default()
            },
            Button::Selected(palette) => button::Appearance {
                background: Some(Background::Color(palette.normal.primary)),
                text_color: palette.bright.primary,
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            Button::NormalText(palette) => button::Appearance {
                text_color: palette.normal.surface,
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            Button::SelectedColumn(palette) => button::Appearance {
                background: Some(Background::Color(palette.base.background)),
                text_color: Color {
                    ..palette.bright.primary
                },
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            Button::ColumnHeader(palette) => button::Appearance {
                background: Some(Background::Color(palette.base.background)),
                text_color: Color {
                    ..palette.bright.surface
                },
                border_radius: 2.0,
                ..button::Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance::default(),
            Button::Bordered(palette) => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..self.active(style)
            },
            Button::Primary(palette) => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..self.active(style)
            },
            Button::Selected(palette) => button::Appearance {
                background: Some(Background::Color(palette.normal.primary)),
                text_color: palette.bright.primary,
                ..self.active(style)
            },
            Button::NormalText(palette) => button::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: palette.bright.primary,
                ..self.active(style)
            },
            Button::SelectedColumn(palette) => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..self.active(style)
            },
            Button::ColumnHeader(palette) => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.15,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..self.active(style)
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance::default(),
            Button::Bordered(palette) => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.05,
                    ..palette.normal.primary
                })),
                text_color: Color {
                    a: 0.50,
                    ..palette.normal.primary
                },
                ..self.active(style)
            },
            Button::Primary(palette) => button::Appearance {
                text_color: Color {
                    a: 0.25,
                    ..palette.normal.surface
                },
                ..self.active(style)
            },
            Button::Selected(palette) => button::Appearance {
                ..self.active(style)
            },
            _ => self.disabled(style),
        }
    }
}
