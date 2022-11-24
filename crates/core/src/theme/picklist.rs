

use iced_style::{pick_list, menu};
use iced::{Background, Color};
use super::{ColorPalette, Theme};


#[derive(Debug, Clone, Copy, Default)]
pub enum PickListStyles {
    #[default]
    Default,
    Primary,
}

impl pick_list::StyleSheet for Theme {
    type Style = PickListStyles;

    fn active(&self, style: &Self::Style) -> pick_list::Appearance {
        match style {
            PickListStyles::Primary => pick_list::Appearance {
                text_color: self.palette.bright.surface,
                background: self.palette.base.background.into(),
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
                border_radius: 2.0,
                icon_size: 0.5,
                placeholder_color: Color {
                    a: 0.5,
                    ..self.palette.normal.primary
                },
            },
            _ => todo!("default")
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        match style {
            PickListStyles::Primary => {
                let active = self.active(style);

                pick_list::Appearance {
                    text_color: self.palette.bright.primary,
                    ..active
                }
            },
            _ => todo!("default")
        }
    }
}


impl menu::StyleSheet for Theme {
    type Style = PickListStyles;

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        match style {
            PickListStyles::Primary => menu::Appearance {  
                text_color: self.palette.bright.surface,
                background: Background::Color(self.palette.base.foreground),
                border_width: 1.0,
                border_radius: 2.0,
                border_color: self.palette.base.background,
                selected_background: Background::Color(Color {
                    a: 0.15,
                    ..self.palette.normal.primary
                }),
                selected_text_color: self.palette.bright.primary,
            },
            _ => todo!("default")

        }
    }
}