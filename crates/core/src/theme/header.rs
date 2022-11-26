use grin_gui_widgets::style::header::{StyleSheet, Style};
use grin_gui_widgets::header;
use iced_graphics::{Backend, Renderer};
use iced_native::mouse;
use iced_native::widget::Tree;
use iced_native::{Element, Layout, Point, Rectangle};
use iced::{Color};
use super::Theme;

impl<B> header::Renderer<Theme> for Renderer<B, Theme>
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
        theme: &Theme,
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


impl header::StyleSheet for Theme {
    fn style(&self) -> Style {
        Style {
            text_color: None,
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
