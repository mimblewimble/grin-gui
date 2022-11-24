use super::Theme;
use iced::{Background, Color};
use iced_aw::style::modal::Appearance;

#[derive(Clone, Copy, Debug, Default)]
pub enum ModalStyle {
    #[default]
    Default,
    Normal,
}

impl iced_aw::modal::StyleSheet for Theme {
    type Style = ModalStyle;

    fn active(&self, style: Self::Style) -> Appearance {
        match style {
            ModalStyle::Normal => Appearance {
                background: Background::Color(Color {
                    a: 0.9,
                    ..self.palette.base.foreground
                }),
            },
            _ => Appearance::default()
        }
    }
}
