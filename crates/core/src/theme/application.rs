use iced::{application, Color};
use super::Theme;

impl application::StyleSheet for Theme {
    type Style = iced_style::theme::Application;

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        let palette = self.palette;

        application::Appearance {
            background_color: palette.base.background,
            text_color: palette.normal.primary,
        }
    }
}