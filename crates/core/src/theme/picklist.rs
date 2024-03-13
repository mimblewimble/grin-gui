use super::Theme;
use iced::{Background, Color};
use iced_core::{Border, Shadow};
use iced_style::{menu, pick_list};

#[derive(Debug, Clone, Copy, Default)]
pub enum PickListStyle {
	#[default]
	Default,
	Primary,
}

impl pick_list::StyleSheet for Theme {
	type Style = PickListStyle;

	fn active(&self, style: &Self::Style) -> pick_list::Appearance {
		match style {
			PickListStyle::Primary => pick_list::Appearance {
				text_color: self.palette.bright.surface,
				background: self.palette.base.background.into(),
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				handle_color: Color {
					a: 0.5,
					..self.palette.normal.primary
				},
				placeholder_color: Color {
					a: 0.5,
					..self.palette.normal.primary
				},
			},
			_ => todo!("default"),
		}
	}

	fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
		match style {
			PickListStyle::Primary => {
				let active = self.active(style);

				pick_list::Appearance {
					text_color: self.palette.bright.primary,
					..active
				}
			}
			_ => todo!("default"),
		}
	}
}

impl menu::StyleSheet for Theme {
	type Style = PickListStyle;

	fn appearance(&self, style: &Self::Style) -> menu::Appearance {
		match style {
			PickListStyle::Primary => menu::Appearance {
				text_color: self.palette.bright.surface,
				background: Background::Color(self.palette.base.foreground),
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				selected_background: Background::Color(Color {
					a: 0.15,
					..self.palette.normal.primary
				}),
				selected_text_color: self.palette.bright.primary,
			},
			_ => todo!("default"),
		}
	}
}
