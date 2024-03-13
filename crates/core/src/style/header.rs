use iced::Theme;
use iced_core::{Background, Color};

/// The appearance of a header.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
	pub text_color: Option<Color>,
	pub background: Option<Background>,
	pub border_radius: f32,
	pub border_width: f32,
	pub border_color: Color,
	pub offset_left: f32,
	pub offset_right: f32,
}

/// A set of rules that dictate the style of a header.
pub trait StyleSheet {
	type Style: std::default::Default + Copy;

	/// Produces the style of a header.
	fn appearance(&self, style: &Self::Style) -> Appearance;

	/// Produces the a hovered appearance for header.
	fn hovered(&self, style: &Self::Style) -> Appearance;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeaderStyle {
	#[default]
	Default,
}

impl StyleSheet for Theme {
	type Style = HeaderStyle;

	fn appearance(&self, style: &Self::Style) -> Appearance {
		match style {
			HeaderStyle::Default => Appearance {
				//text_color: Some(self.palette.bright.surface),
				text_color: None,
				background: Some(Background::Color(self.palette().primary)),
				border_radius: 0.0,
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				offset_right: 0.0,
				offset_left: 0.0,
			},
		}
	}

	fn hovered(&self, style: &Self::Style) -> Appearance {
		let appearance = self.appearance(style);
		Appearance {
			background: None,
			..appearance
		}
	}
}
