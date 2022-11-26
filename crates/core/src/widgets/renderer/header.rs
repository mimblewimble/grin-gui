use super::super::style::header::StyleSheet;
use super::super::widget::header;
use iced_graphics::{Backend, Renderer};
use iced_native::mouse;
use iced_native::widget::Tree;
use iced_native::{Element, Layout, Point, Rectangle};
use crate::theme::Theme;

impl<B> header::Renderer for Renderer<B, Theme>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn mouse_interaction(
        &self,
        tree: &Tree,
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
        theme: &crate::theme::Theme,
        cursor_position: Point,
        _style_sheet: &dyn StyleSheet,
        content: &Vec<Element<'_, Message, Self>>,
        viewport: &Rectangle,
        _custom_bounds: &Rectangle,
    ) {
        for (child, layout) in content.iter().zip(layout.children()) {
            child.as_widget().draw(tree, self, theme, &iced_native::renderer::Style::default(), layout, cursor_position, viewport);
        }
    }
}