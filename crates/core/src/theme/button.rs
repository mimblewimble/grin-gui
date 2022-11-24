use iced::widget::button;
use iced::{Background, Color};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
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
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance::default(),
            Button::Bordered => button::Appearance {
                border_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
                border_width: 1.0,
                border_radius: 2.0,
                text_color: self.palette.bright.primary,
                ..button::Appearance::default()
            },
            Button::Primary => button::Appearance {
                text_color: self.palette.bright.primary,
                border_radius: 2.0,
                ..Default::default()
            },
            Button::Selected => button::Appearance {
                background: Some(Background::Color(self.palette.normal.primary)),
                text_color: self.palette.bright.primary,
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            Button::NormalText =>  button::Appearance {
                text_color: self.palette.normal.surface,
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            Button::SelectedColumn => button::Appearance {
                background: Some(Background::Color(self.palette.base.background)),
                text_color: Color {
                    ..self.palette.bright.primary
                },
                border_radius: 2.0,
                ..button::Appearance::default()
            },
            Button::ColumnHeader => button::Appearance {
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
            Button::Default => button::Appearance::default(),
            Button::Bordered => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            Button::Primary => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            Button::Selected => button::Appearance {
                background: Some(Background::Color(self.palette.normal.primary)),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            Button::NormalText => button::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            Button::SelectedColumn => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                text_color: self.palette.bright.primary,
                ..self.active(style)
            },
            Button::ColumnHeader => button::Appearance {
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
            Button::Default => button::Appearance::default(),
            Button::Bordered => button::Appearance {
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
            Button::Primary => button::Appearance {
                text_color: Color {
                    a: 0.25,
                    ..self.palette.normal.surface
                },
                ..self.active(style)
            },
            Button::Selected => button::Appearance {
                ..self.active(style)
            },
            _ => self.disabled(style),
        }
    }
}
