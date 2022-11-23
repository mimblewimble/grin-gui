use super::{ColorPalette, Theme};
use iced::{Background, Color};
use iced_aw::style::modal::Appearance;

#[derive(Clone, Copy, Debug, Default)]
pub enum ModalStyles {
    #[default]
    Default,

    Normal(ColorPalette),
}

impl iced_aw::modal::StyleSheet for Theme {
    type Style = ModalStyles;

    fn active(&self, style: Self::Style) -> Appearance {
        match style {
            ModalStyles::Normal(palette) => Appearance {
                background: Background::Color(Color {
                    a: 0.9,
                    ..palette.base.foreground
                }),
            },
            _ => Appearance::default()
        }
    }
}
