use super::Theme;
use iced::Color;
use iced_style::radio;

#[derive(Debug, Clone, Copy, Default)]
pub enum RadioStyle {
	#[default]
	Default,
	Primary,
}

impl radio::StyleSheet for Theme {
	type Style = RadioStyle;

	fn active(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
		match style {
			RadioStyle::Primary => radio::Appearance {
				text_color: Some(self.palette.bright.surface),
				background: self.palette.base.background.into(),
				dot_color: self.palette.bright.surface,
				border_width: 1.0,
				border_color: Color {
					a: 0.5,
					..self.palette.normal.primary
				},
			},
			_ => todo!("default"),
		}
	}

	fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
		match style {
			RadioStyle::Primary => {
				let active = self.active(style, is_selected);

				radio::Appearance {
					text_color: Some(self.palette.bright.primary),
					..active
				}
			}
			_ => todo!("default"),
		}
	}
}

/*impl menu::StyleSheet for Theme {
	type Style = PickListStyle;

	fn appearance(&self, style: &Self::Style) -> menu::Appearance {
		match style {
			PickListStyle::Primary => menu::Appearance {
				text_color: self.palette.bright.surface,
				background: Background::Color(self.palette.base.foreground),
				border_width: 1.0,
				border_radius: 2.0.into(),
				border_color: self.palette.base.background,
				selected_background: Background::Color(Color {
					a: 0.15,
					..self.palette.normal.primary
				}),
				selected_text_color: self.palette.bright.primary,
			},
			_ => todo!("default")

		}
	}
}*/
