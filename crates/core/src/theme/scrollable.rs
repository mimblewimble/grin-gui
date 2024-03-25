use super::Theme;
use iced::widget::scrollable;
use iced::{Background, Color};
use iced_style::scrollable::{Appearance, Scrollbar, Scroller};

#[derive(Debug, Clone, Copy, Default)]
pub enum ScrollableStyle {
	#[default]
	Default,
	Primary,
}

impl scrollable::StyleSheet for Theme {
	type Style = ScrollableStyle;

	fn active(&self, style: &Self::Style) -> Appearance {
		let mut appearance = Appearance {
			container: Default::default(),
			scrollbar: Scrollbar {
				background: None,
				border: Default::default(),
				scroller: Scroller {
					color: self.palette.base.background,
					border: Default::default(),
				},
			},
			gap: None,
		};

		match style {
			ScrollableStyle::Default => {
				appearance.scrollbar.background = Some(Background::Color(Color::TRANSPARENT));
				appearance.scrollbar.border.radius = 0.0.into();
				appearance.scrollbar.border.width = 0.0.into();
				appearance.scrollbar.border.color = Color::TRANSPARENT;
			}

			ScrollableStyle::Primary => {
				appearance.scrollbar.background =
					Some(Background::Color(self.palette.base.background));
				appearance.scrollbar.border.radius = 0.0.into();
				appearance.scrollbar.border.width = 0.0.into();
				appearance.scrollbar.border.color = Color::TRANSPARENT;
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
