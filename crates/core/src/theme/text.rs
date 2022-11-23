use iced::widget::text;
use super::Theme;


impl text::StyleSheet for Theme {
    type Style = iced_style::theme::Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            iced_style::theme::Text::Default => Default::default(),
            iced_style::theme::Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

