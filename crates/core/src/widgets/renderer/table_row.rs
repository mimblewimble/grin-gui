use crate::{theme::Theme};
use super::super::widget::table_row;
use iced_graphics::{Backend, Renderer};
use iced_native::{
    mouse, widget::Tree, Background, Color, Element, Layout, Point, Rectangle,
    Renderer as iced_native_Renderer,
};
pub use super::super::style::table_row::{Appearance, StyleSheet};

impl<B> table_row::Renderer for Renderer<B, Theme>
where
    B: Backend,
{
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
        style: &iced_native::renderer::Style,
        style_sheet: &dyn StyleSheet,
        content: &Element<'_, Message, Self>,
        viewport: &Rectangle,
        custom_bounds: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let is_mouse_over = custom_bounds.contains(cursor_position);
        let content_layout = layout.children().next().unwrap();

        let appearance = if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.style()
        };

        let background = iced_native::renderer::Quad {
            bounds: Rectangle {
                x: bounds.x + appearance.offset_left as f32,
                y: bounds.y,
                width: bounds.width - appearance.offset_right as f32,
                height: custom_bounds.height,
            },
            border_radius: appearance.border_radius,
            border_width: appearance.border_width,
            border_color: appearance.border_color,
        };

        self.fill_quad(
            background.into(),
            appearance
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
            style,
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
