use crate::theme::Theme;
use iced::{Background, Color};

/// The appearance of a table row.
#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
	pub text_color: Option<Color>,
	pub background: Option<Background>,
	pub border_radius: f32,
	pub border_width: f32,
	pub border_color: Color,
	pub offset_left: f32,
	pub offset_right: f32,
}

/// A set of rules that dictate the style of a table row.
pub trait StyleSheet {
	type Style: std::default::Default + Copy;

	/// Produces the default appearance of a table row.
	fn appearance(&self, style: &Self::Style) -> Appearance;

	/// Produces the hovered appearance table row.
	fn hovered(&self, style: &Self::Style) -> Appearance;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableRowStyle {
	#[default]
	Default,
	TableRowAlternate,
	TableRowHighlife,
	TableRowLowlife,
	TableRowSelected,
}

impl StyleSheet for Theme {
	type Style = TableRowStyle;

	fn appearance(&self, style: &Self::Style) -> Appearance {
		match style {
			TableRowStyle::Default => Appearance {
				text_color: Some(self.palette.normal.primary),
				background: Some(Background::Color(self.palette.normal.primary)),
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				// offset_left: 10.0,
				// offset_right: 25.0,
				offset_left: 0.0,
				offset_right: 0.0,
			},
			TableRowStyle::TableRowAlternate => Appearance {
				background: Some(Background::Color(Color {
					a: 0.50,
					..self.palette.normal.primary
				})),
				..Appearance::default()
			},
			TableRowStyle::TableRowHighlife => Appearance {
				text_color: Some(self.palette.normal.primary),
				background: Some(Background::Color(Color {
					a: 0.30,
					..self.palette.normal.primary
				})),
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				offset_left: 0.0,
				offset_right: 0.0,
			},
			TableRowStyle::TableRowLowlife => Appearance {
				text_color: Some(self.palette.normal.primary),
				background: Some(Background::Color(Color::TRANSPARENT)),
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				offset_left: 0.0,
				offset_right: 0.0,
			},
			TableRowStyle::TableRowSelected => Appearance {
				text_color: Some(self.palette.normal.primary),
				background: Some(Background::Color(self.palette.normal.primary)),
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				offset_left: 0.0,
				offset_right: 0.0,
			},
		}
	}

	fn hovered(&self, style: &Self::Style) -> Appearance {
		let appearance = self.appearance(style);

		match style {
			TableRowStyle::Default => Appearance {
				background: Some(Background::Color(Color {
					a: 0.60,
					..self.palette.normal.primary
				})),
				..appearance
			},
			TableRowStyle::TableRowAlternate => Appearance {
				background: Some(Background::Color(Color {
					a: 0.25,
					..self.palette.normal.primary
				})),
				..appearance
			},
			TableRowStyle::TableRowHighlife => Appearance {
				background: Some(Background::Color(Color {
					a: 0.60,
					..self.palette.normal.primary
				})),
				..appearance
			},
			TableRowStyle::TableRowLowlife => Appearance {
				background: Some(Background::Color(Color {
					a: 0.60,
					..self.palette.normal.primary
				})),
				..appearance
			},
			TableRowStyle::TableRowSelected => Appearance {
				background: Some(Background::Color(Color {
					a: 0.60,
					..self.palette.normal.primary
				})),
				..appearance
			},
		}
	}
}
