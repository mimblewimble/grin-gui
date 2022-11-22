use super::ColorPalette;
use super::Theme;
use iced::widget::checkbox;
use iced::Background;

#[derive(Debug, Clone, Copy, Default)]
pub enum CheckboxStyles {
    #[default]
    Default,
    Normal(ColorPalette),
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckboxStyles;

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckboxStyles::Normal(palette) => checkbox::Appearance {
                background: Background::Color(palette.base.background),
                checkmark_color: palette.bright.primary,
                border_radius: 2.0,
                border_width: 1.0,
                border_color: palette.normal.primary,
                text_color: Some(palette.normal.surface),
            },
            _ => todo!("default"),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckboxStyles::Normal(palette) => checkbox::Appearance {
                background: Background::Color(palette.base.foreground),
                checkmark_color: palette.bright.primary,
                border_radius: 2.0,
                border_width: 2.0,
                border_color: palette.bright.primary,
                text_color: Some(palette.normal.surface),
            },
            _ => todo!("default"),
        }
    }
}
