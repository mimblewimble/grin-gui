use super::Theme;
use iced::widget::text_editor;
use iced::{Background, Color};
use iced_core::Border;

#[derive(Debug, Clone, Copy, Default)]
pub enum TextEditorStyle {
	#[default]
	Default,
}

impl text_editor::StyleSheet for Theme {
	type Style = TextEditorStyle;

	/// Produces the style of an active text input.
	fn active(&self, style: &Self::Style) -> text_editor::Appearance {
		match style {
			TextEditorStyle::Default => text_editor::Appearance {
				background: Background::Color(self.palette.base.foreground),
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
			},
		}
	}

	/// Produces the style of a focused text input.
	fn focused(&self, style: &Self::Style) -> text_editor::Appearance {
		match style {
			TextEditorStyle::Default => text_editor::Appearance {
				background: Background::Color(self.palette.base.foreground),
				border: Border {
					color: self.palette.bright.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
			},
		}
	}

	fn disabled(&self, style: &Self::Style) -> text_editor::Appearance {
		match style {
			TextEditorStyle::Default => text_editor::Appearance {
				background: Background::Color(self.palette.base.foreground),
				border: Border {
					color: self.palette.normal.primary,
					width: 1.0,
					radius: 2.0.into(),
				},
			},
		}
	}

	fn placeholder_color(&self, style: &Self::Style) -> Color {
		match style {
			TextEditorStyle::Default => self.palette.normal.surface,
			_ => todo!("default"),
		}
	}

	fn value_color(&self, style: &Self::Style) -> Color {
		match style {
			TextEditorStyle::Default => self.palette.bright.primary,
			_ => todo!("default"),
		}
	}

	fn selection_color(&self, style: &Self::Style) -> Color {
		match style {
			TextEditorStyle::Default => self.palette.bright.secondary,
			_ => todo!("default"),
		}
	}

	fn disabled_color(&self, style: &Self::Style) -> Color {
		match style {
			TextEditorStyle::Default => self.palette.normal.secondary,
			_ => todo!("default"),
		}
	}

	/// Produces the style of an hovered text editor.
	fn hovered(&self, style: &Self::Style) -> text_editor::Appearance {
		self.focused(style)
	}
}
