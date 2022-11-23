//use crate::style::StyleSheet;
use super::ColorPalette;
use super::Theme;
use grin_gui_widgets::widget::table_row::{Style, StyleSheet};
use iced_graphics::{Backend, Renderer};
use iced_native::{
    mouse, widget::Tree, Background, Color, Element, Layout, Point, Rectangle,
    Renderer as iced_native_Renderer,
};

impl StyleSheet for Theme {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::WHITE),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_right: 0.0,
            offset_left: 0.0,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            background: None,
            ..self.style()
        }
    }
}


pub struct TableRowAlternate(pub ColorPalette);
impl StyleSheet for TableRowAlternate {
    fn style(&self) -> Style {
        let default = TableRow(self.0).style();

        Style {
            background: Some(Background::Color(Color {
                a: 0.50,
                ..self.0.base.foreground
            })),
            ..default
        }
    }
    fn hovered(&self) -> Style {
        let style = self.style();
        Style {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct TableRow(pub ColorPalette);
impl StyleSheet for TableRow {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Color(self.0.base.foreground)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 10.0,
            offset_right: 25.0,
        }
    }
    fn hovered(&self) -> Style {
        let style = self.style();
        Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct TableRowHighlife(pub ColorPalette);
impl StyleSheet for TableRowHighlife {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Color(Color {
                a: 0.30,
                ..self.0.base.foreground
            })),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> Style {
        let style = self.style();
        Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct TableRowLowlife(pub ColorPalette);
impl StyleSheet for TableRowLowlife {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> Style {
        let style = self.style();
        Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}
pub struct TableRowSelected(pub ColorPalette);
impl StyleSheet for TableRowSelected {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Color(self.0.normal.primary)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> Style {
        let style = self.style();
        Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

impl<B> grin_gui_widgets::widget::table_row::Renderer<Theme> for Renderer<B, Theme>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw<Message>(
        &mut self,
        tree: &Tree,
        layout: Layout<'_>,
        theme: &Theme,
        cursor_position: Point,
        style_sheet: &dyn StyleSheet,
        content: &Element<'_, Message, Self>,
        viewport: &Rectangle,
        custom_bounds: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let is_mouse_over = custom_bounds.contains(cursor_position);
        let content_layout = layout.children().next().unwrap();

        let style = if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.style()
        };

        let background = iced_native::renderer::Quad {
            bounds: Rectangle {
                x: bounds.x + style.offset_left as f32,
                y: bounds.y,
                width: bounds.width - style.offset_right as f32,
                height: custom_bounds.height,
            },
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        };

        self.fill_quad(
            background.into(),
            style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        /*self.fill_quad(
            renderer::Quad {
                bounds,
                border_color: style.border_color,
                border_width: style.border_width,
                border_radius: style.border_radius,
            },
            style.background,
        );*/

        content.as_widget().draw(
            tree,
            self,
            theme,
            &iced_native::renderer::Style::default(),
            content_layout,
            cursor_position,
            viewport,
        );

        //content.draw(self, style.into().as_ref(), content_layout, cursor_position, viewport);

        /*(
            if style.background.is_some() {
                let background = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds.x + style.offset_left as f32,
                        y: bounds.y,
                        width: bounds.width - style.offset_right as f32,
                        height: custom_bounds.height,
                    },
                    background: style
                        .background
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                };

                Primitive::Group {
                    primitives: vec![background, content],
                }
            } else {
                content
            },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse_interaction
            },
        )*/
    }
}
