use super::Theme;
use iced::widget::checkbox;
use iced::Background;
use iced_core::Border;

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
			CheckboxStyle::Normal => checkbox::Appearance {
				background: Background::Color(self.palette.base.background),
				icon_color: self.palette.bright.primary,
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				text_color: Some(self.palette.normal.surface),
			},
			_ => todo!("default"),
		}
	}

	fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
		match style {
			CheckboxStyle::Normal => checkbox::Appearance {
				background: Background::Color(self.palette.base.foreground),
				icon_color: self.palette.bright.primary,
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				text_color: Some(self.palette.normal.surface),
			},
			_ => todo!("default"),
		}
	}
}
