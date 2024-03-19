use super::Theme;
use iced::widget::text;

#[derive(Debug, Clone, Copy, Default)]
pub enum TextStyle {
	#[default]
	Default,
	Warning,
}

impl text::StyleSheet for Theme {
	type Style = TextStyle;

	fn appearance(&self, style: Self::Style) -> text::Appearance {
		match style {
			TextStyle::Warning => text::Appearance {
				color: Some(self.palette.bright.error),
			},
			TextStyle::Default => Default::default(),
		}
	}
}
