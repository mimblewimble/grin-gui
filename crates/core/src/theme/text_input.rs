use super::ColorPalette;
use super::Theme;
use iced::widget::text_input;
use iced::{Background, Color};

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInputStyles {
    #[default]
    Default,

    AddonsQuery(ColorPalette),
}

impl text_input::StyleSheet for Theme {
    type Style = TextInputStyles;

    /// Produces the style of an active text input.
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInputStyles::AddonsQuery(palette) => text_input::Appearance {
                background: Background::Color(palette.base.foreground),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: palette.base.foreground,
            },
            _ => todo!("default"),
        }
    }

    /// Produces the style of a focused text input.
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInputStyles::AddonsQuery(palette) => text_input::Appearance {
                background: Background::Color(palette.base.foreground),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..palette.normal.primary
                },
            },
            _ => todo!("default"),
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInputStyles::AddonsQuery(palette) => palette.normal.surface, 
            _ => todo!("default"),
        }
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInputStyles::AddonsQuery(palette) => palette.bright.primary, 
            _ => todo!("default"),
        }
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInputStyles::AddonsQuery(palette) => palette.bright.secondary, 
            _ => todo!("default"),
        }
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}
