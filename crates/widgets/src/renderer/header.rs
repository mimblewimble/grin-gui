use crate::style::header::StyleSheet;

use crate::widget::header;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::mouse;
use iced_native::{Element, Layout, Point, Rectangle, Renderer as iced_native_Renderer};

impl<B> header::Renderer for Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
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
        layout: Layout<'_>,
        cursor_position: Point,
        style_sheet: &dyn StyleSheet,
        content: &Vec<Element<'_, Message, Self>>,
        viewport: &Rectangle,
        custom_bounds: &Rectangle,
    ) {
        for (child, layout) in content.iter().zip(layout.children()) {
            child.draw(self, &iced_native::renderer::Style::default(), layout, cursor_position, viewport);
        }
    }
}
