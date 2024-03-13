use super::Theme;
use iced::widget::container;
use iced::{Background, Color};
use iced_core::{Border, Shadow, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerStyle {
	#[default]
	Default,
	BrightForeground,
	BrightBackground,
	ErrorForeground,
	NormalBackground,
	HoverableForeground,
	HoverableBrightForeground,
	SuccessBackground,
	Segmented,
	PanelBordered,
	PanelForeground,
}

impl container::StyleSheet for Theme {
	type Style = ContainerStyle;

	fn appearance(&self, style: &Self::Style) -> container::Appearance {
		match style {
			ContainerStyle::Default => container::Appearance::default(),
			ContainerStyle::BrightBackground => container::Appearance {
				background: Some(Background::Color(self.palette.base.background)),
				text_color: Some(self.palette.bright.surface),
				..container::Appearance::default()
			},
			ContainerStyle::BrightForeground => container::Appearance {
				background: Some(Background::Color(self.palette.base.foreground)),
				text_color: Some(self.palette.bright.surface),
				..container::Appearance::default()
			},
			ContainerStyle::ErrorForeground => container::Appearance {
				background: Some(Background::Color(self.palette.base.foreground)),
				text_color: Some(self.palette.normal.surface),
				..container::Appearance::default()
			},
			ContainerStyle::NormalBackground => container::Appearance {
				background: Some(Background::Color(self.palette.base.background)),
				text_color: Some(self.palette.normal.surface),
				..container::Appearance::default()
			},
			ContainerStyle::Segmented => container::Appearance {
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				..container::Appearance::default()
			},
			ContainerStyle::HoverableForeground => container::Appearance {
				background: None,
				text_color: Some(self.palette.normal.surface),
				..container::Appearance::default()
			},
			ContainerStyle::HoverableBrightForeground => container::Appearance {
				background: None,
				text_color: Some(self.palette.bright.primary),
				..container::Appearance::default()
			},
			ContainerStyle::SuccessBackground => container::Appearance {
				background: Some(Background::Color(self.palette.base.foreground)),
				text_color: Some(self.palette.normal.surface),
				..container::Appearance::default()
			},
			ContainerStyle::PanelForeground => container::Appearance {
				background: Some(Background::Color(self.palette.base.foreground)),
				text_color: Some(self.palette.bright.primary),
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				shadow: Shadow {
					offset: Vector::new(0.0, 1.0),
					blur_radius: 1.0,
					color: Color::TRANSPARENT,
				},
			},
			ContainerStyle::PanelBordered => container::Appearance {
				background: Some(Background::Color(Color::TRANSPARENT)),
				text_color: Some(self.palette.bright.primary),
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
				shadow: Shadow {
					offset: Vector::new(0.0, 1.0),
					blur_radius: 1.0,
					color: Color::TRANSPARENT,
				},
			},
		}
	}
}
