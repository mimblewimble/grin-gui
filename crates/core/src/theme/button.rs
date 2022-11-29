use iced::widget::button;
use iced::{Background, Color};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    Bordered,
    ColumnHeader,
    Primary,
    Selected,
    SelectedColumn,
    NormalText,
}

impl button::StyleSheet for Theme {
    type Style = ButtonStyle;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Default => button::Appearance::default(),
            ButtonStyle::Bordered => button::Appearance {
                border_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
                border_width: 1.0,
                border_radius: 2.0,
                text_color: self.palette.bright.primary,
                ..button::Appearance::default()
            },
            ButtonStyle::Primary => button::Appearance {
                text_color: self.palette.bright.primary,
                border_radius: 2.0,
                ..Default::default()
            },
            ButtonStyle::Selected => button::Appearance {
                background: Some(Background::Color(self.palette.normal.primary)),
                text_color: self.palette.bright.primary,
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            ButtonStyle::NormalText =>  button::Appearance {
                text_color: self.palette.normal.surface,
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            ButtonStyle::SelectedColumn => button::Appearance {
                background: Some(Background::Color(self.palette.base.background)),
                text_color: Color {
                    ..self.palette.bright.primary
                },
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            ButtonStyle::ColumnHeader => button::Appearance {
                background: Some(Background::Color(self.palette.base.background)),
                text_color: Color {
                    ..self.palette.bright.surface
                },
                border_radius: 2.0,
                ..button::Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Default => button::Appearance::default(),
            ButtonStyle::Bordered => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            ButtonStyle::Primary => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            ButtonStyle::Selected => button::Appearance {
                background: Some(Background::Color(self.palette.normal.primary)),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            ButtonStyle::NormalText => button::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            ButtonStyle::SelectedColumn => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            ButtonStyle::ColumnHeader => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.15,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Default => button::Appearance::default(),
            ButtonStyle::Bordered => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.05,
                    ..self.palette.normal.primary
                })),
                text_color: Color {
                    a: 0.50,
                    ..self.palette.normal.primary
                },
                ..self.active(style)
            },
            ButtonStyle::Primary => button::Appearance {
                text_color: Color {
                    a: 0.25,
                    ..self.palette.normal.surface
                },
                ..self.active(style)
            },
            ButtonStyle::Selected => button::Appearance {
                ..self.active(style)
            },
            _ => self.disabled(style),
        }
    }
}
