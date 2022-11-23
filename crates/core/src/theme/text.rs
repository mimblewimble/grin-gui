use iced::widget::text;
use super::Theme;


impl text::StyleSheet for Theme {
    type Style = iced_style::theme::Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        Default::default()
    }
}

