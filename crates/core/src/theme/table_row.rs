use super::Theme;
use crate::widgets::table_row::Appearance;
use crate::widgets::style::table_row::StyleSheetAssociated;
use iced_native::{Background, Color};


#[derive(Debug, Clone, Copy, Default)]
pub enum TableRowStyle {
    #[default]
    Default,
    TableRowAlternate,
    TableRowHighlife,
    TableRowLowlife,
    TableRowSelected,
}

impl StyleSheetAssociated for Theme {
    type Style = TableRowStyle;

    fn appearance(&self, style: Self::Style) -> Appearance {
        match style {
            TableRowStyle::Default => Appearance {
                text_color: Some(self.palette.normal.primary),
                background: Some(Background::Color(self.palette.base.foreground)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 10.0,
                offset_right: 25.0,
            },
            TableRowStyle::TableRowAlternate => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.50,
                    ..self.palette.base.foreground
                })),
                ..Appearance::default()
            },
            TableRowStyle::TableRowHighlife => Appearance {
                text_color: Some(self.palette.normal.primary),
                background: Some(Background::Color(Color {
                    a: 0.30,
                    ..self.palette.base.foreground
                })),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
            TableRowStyle::TableRowLowlife => Appearance {
                text_color: Some(self.palette.normal.primary),
                background: Some(Background::Color(Color::TRANSPARENT)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
            TableRowStyle::TableRowSelected => Appearance {
                text_color: Some(self.palette.normal.primary),
                background: Some(Background::Color(self.palette.normal.primary)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
        }
    }

    fn hovered(&self, style: Self::Style) -> Appearance {
        let appearance = self.appearance(style);

        match style {
            TableRowStyle::Default => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..self.palette.normal.primary
                })),
                ..appearance
            },
            TableRowStyle::TableRowAlternate => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..self.palette.normal.primary
                })),
                ..appearance
            },
            TableRowStyle::TableRowHighlife => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..self.palette.normal.primary
                })),
                ..appearance
            },
            TableRowStyle::TableRowLowlife => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..self.palette.normal.primary
                })),
                ..appearance
            },
            TableRowStyle::TableRowSelected => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..self.palette.normal.primary
                })),
                ..appearance
            },
        }
    }
}
