use super::Theme;
use iced::widget::text;

impl text::StyleSheet for Theme {
    type Style = iced_style::theme::Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        // text::Appearance {
        //     color: Some(self.palette.bright.surface),
        // }
        Default::default()
    }
}
