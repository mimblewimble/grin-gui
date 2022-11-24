use super::Theme;
use iced::{Background, Color};
use iced_aw::style::modal::Appearance;

#[derive(Clone, Copy, Debug, Default)]
pub enum ModalStyles {
    #[default]
    Default,
    Normal,
}

impl iced_aw::modal::StyleSheet for Theme {
    type Style = ModalStyles;

    fn active(&self, style: Self::Style) -> Appearance {
        match style {
            ModalStyles::Normal => Appearance {
                background: Background::Color(Color {
                    a: 0.9,
                    ..self.palette.base.foreground
                }),
            },
            _ => Appearance::default()
        }
    }
}
