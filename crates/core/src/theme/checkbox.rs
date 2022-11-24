use super::ColorPalette;
use super::Theme;
use iced::widget::checkbox;
use iced::Background;

#[derive(Debug, Clone, Copy, Default)]
pub enum CheckboxStyle {
    #[default]
    Default,
    Normal,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckboxStyle;

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckboxStyle::Normal =>  checkbox::Appearance {
                background: Background::Color(self.palette.base.background),
                checkmark_color: self.palette.bright.primary,
                border_radius: 2.0,
                border_width: 1.0,
                border_color: self.palette.normal.primary,
                text_color: Some(self.palette.normal.surface),
            },
            _ => todo!("default"),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckboxStyle::Normal => checkbox::Appearance {
                background: Background::Color(self.palette.base.foreground),
                checkmark_color: self.palette.bright.primary,
                border_radius: 2.0,
                border_width: 2.0,
                border_color: self.palette.bright.primary,
                text_color: Some(self.palette.normal.surface),
            },
            _ => todo!("default"),
        }
    }
}
