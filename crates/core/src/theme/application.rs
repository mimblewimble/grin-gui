use super::Theme;
use iced::application;

impl application::StyleSheet for Theme {
	type Style = iced_style::theme::Application;

	fn appearance(&self, style: &Self::Style) -> application::Appearance {
		application::Appearance {
			background_color: self.palette.base.background,
			text_color: self.palette.normal.primary,
		}
	}
}
