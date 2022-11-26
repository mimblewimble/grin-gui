use super::Theme;
use iced::widget::text_input;
use iced::{Background, Color};

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInputStyle {
    #[default]
    Default,
    AddonsQuery,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInputStyle;

    /// Produces the style of an active text input.
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInputStyle::AddonsQuery => text_input::Appearance {
                background: Background::Color(self.palette.base.foreground),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: self.palette.base.foreground,
            },
            _ => todo!("default"),
        }
    }

    /// Produces the style of a focused text input.
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInputStyle::AddonsQuery => text_input::Appearance {
                background: Background::Color(self.palette.base.foreground),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
            },
            _ => todo!("default"),
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInputStyle::AddonsQuery => self.palette.normal.surface, 
            _ => todo!("default"),
        }
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInputStyle::AddonsQuery => self.palette.bright.primary, 
            _ => todo!("default"),
        }
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInputStyle::AddonsQuery => self.palette.bright.secondary, 
            _ => todo!("default"),
        }
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}
