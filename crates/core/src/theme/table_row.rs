use iced::{Background, Color};
use iced_aw::style::table_row::{Appearance, RowOrCellAppearance};
use iced_aw::table;

use super::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableRowStyle {
	#[default]
	Default,
	TableRowAlternate,
	TableRowHighlight,
	TableRowLowlight,
	TableRowSelected,
}

impl table::TableRowStyleSheet for Theme {
	type Style = TableRowStyle;
	fn appearance(&self, style: &Self::Style, row_id: u16) -> Appearance {
		let palette = self.palette;

		match style {
			TableRowStyle::Default => Appearance {
				row: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(palette.base.foreground)),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
				cell: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(palette.base.foreground)),
					border_radius: 0.0.into(),
					border_width: 1.0,
					border_color: Color::BLACK,
					offset_left: 0.0,
					offset_right: 0.0,
				},
			},
			TableRowStyle::TableRowAlternate => Appearance {
				row: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(Color {
						a: 0.50,
						..palette.base.foreground
					})),
					..RowOrCellAppearance::default()
				},
				cell: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(Color {
						a: 0.50,
						..palette.normal.primary
					})),
					..RowOrCellAppearance::default()
				},
			},
			TableRowStyle::TableRowHighlight => Appearance {
				row: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(Color {
						a: 0.30,
						..palette.base.foreground
					})),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
				cell: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(Color {
						a: 0.30,
						..palette.base.foreground
					})),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
			},
			TableRowStyle::TableRowLowlight => Appearance {
				row: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(Color::TRANSPARENT)),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
				cell: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(Color::TRANSPARENT)),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
			},
			TableRowStyle::TableRowSelected => Appearance {
				row: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(palette.normal.primary)),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
				cell: RowOrCellAppearance {
					text_color: palette.normal.primary,
					background: Some(Background::Color(palette.normal.primary)),
					border_radius: 0.0.into(),
					border_width: 0.0,
					border_color: Color::TRANSPARENT,
					offset_left: 0.0,
					offset_right: 0.0,
				},
			},
		}
	}

	fn hovered(&self, style: &Self::Style, row_id: u16) -> Appearance {
		let palette = self.palette;
		match style {
			TableRowStyle::Default => Appearance {
				row: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).row
				},
				cell: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).cell
				},
			},
			TableRowStyle::TableRowAlternate => Appearance {
				row: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.25,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).row
				},
				cell: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.25,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).cell
				},
			},
			TableRowStyle::TableRowHighlight => Appearance {
				row: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).row
				},
				cell: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).cell
				},
			},
			TableRowStyle::TableRowLowlight => Appearance {
				row: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).row
				},
				cell: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).cell
				},
			},
			TableRowStyle::TableRowSelected => Appearance {
				row: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).row
				},
				cell: RowOrCellAppearance {
					background: Some(Background::Color(Color {
						a: 0.60,
						..palette.normal.primary
					})),
					..self.appearance(style, row_id).cell
				},
			},
		}
	}
}
