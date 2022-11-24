use iced::Background;
use iced_aw::card;
use iced_aw::style::card::Appearance;

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum CardStyles {
    #[default]
    Default,
    Normal,
}

impl card::StyleSheet for Theme {
    type Style = CardStyles;

    fn active(&self, style: Self::Style) -> Appearance {
        match style {
            CardStyles::Default => iced_aw::style::card::Appearance::default(),
            CardStyles::Normal => Appearance {
                background: Background::Color(self.palette.base.background),
                head_background: Background::Color(self.palette.normal.primary),
                head_text_color: self.palette.bright.surface,
                border_color: self.palette.normal.primary,
                body_text_color: self.palette.normal.surface,
                border_radius: 5.0,
                ..card::Appearance::default()
            },
        }
    }
}
