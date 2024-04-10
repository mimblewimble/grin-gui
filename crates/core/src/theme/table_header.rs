use iced::{Background, Color};
use iced_aw::style::table_header::Appearance;
use iced_aw::table;

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum TableHeaderStyle {
	#[default]
	Default,
}

impl table::TableHeaderStyleSheet for Theme {
	type Style = TableHeaderStyle;

	fn appearance(&self, style: &Self::Style) -> Appearance {
		let palette = self.palette;

		match style {
			TableHeaderStyle::Default => Appearance {
				//text_color: Some(self.palette.bright.surface),
				text_color: palette.base.foreground,
				background: Some(Background::Color(palette.base.background)),
				border_radius: 0.0.into(),
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				offset_right: 0.0,
				offset_left: 0.0,
			},
		}
	}

	fn hovered(&self, style: &Self::Style) -> Appearance {
		let palette = self.palette;
		match style {
			TableHeaderStyle::Default => Appearance {
				background: None,
				..Appearance::default()
			},
		}
	}
}
