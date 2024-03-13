use super::Theme;
use iced::widget::scrollable;
use iced::{Background, Color};
use iced_style::scrollable::Appearance;

#[derive(Debug, Clone, Copy, Default)]
pub enum ScrollableStyle {
	#[default]
	Default,
	Primary,
}

impl scrollable::StyleSheet for Theme {
	type Style = ScrollableStyle;

	fn active(&self, style: &Self::Style) -> Appearance {
		let mut appearance = self.active(style);

		match style {
			ScrollableStyle::Default => {
				appearance.background = Some(Background::Color(Color::TRANSPARENT));
				appearance.border_radius = 0.0.into();
				appearance.border_width = 0.0.into();
				appearance.border_color = Color::TRANSPARENT;
			}

			ScrollableStyle::Primary => {
				appearance.background = Some(Background::Color(self.palette.base.background));
				appearance.border_radius = 0.0.into();
				appearance.border_width = 0.0.into();
				appearance.border_color = Color::TRANSPARENT;
			}
		}

		appearance
	}

	fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Appearance {
		let active = self.active(style);
		active
	}

	fn dragging(&self, style: &Self::Style) -> Appearance {
		let hovered = self.hovered(style, true);
		hovered
	}
}
