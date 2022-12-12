use super::Theme;
use iced::widget::text;

impl text::StyleSheet for Theme {
    type Style = iced_style::theme::Text;

    // TODO extend the color palette to support text colors
    fn appearance(&self, style: Self::Style) -> text::Appearance {
        //text::Appearance {
        //    color: Some(self.palette.text.primary),
        //}
        Default::default()
    }
}
