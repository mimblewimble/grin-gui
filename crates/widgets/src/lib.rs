#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "wgpu")]
use iced_wgpu::Renderer;

// #[cfg(feature = "opengl")]
// use iced_glow::Renderer;
use grin_gui_core::theme::Renderer;
use grin_gui_core::theme::Theme;
use iced_core::{Background, Color};

mod renderer;
pub mod style;
pub mod widget;

pub use widget::header;
pub use widget::table_row;

pub type Header<'a, Message> = widget::header::Header<'a, Message, Renderer>;
pub type TableRow<'a, Message> = widget::table_row::TableRow<'a, Message, Renderer>;

impl header::StyleSheet for Theme {
    fn style(&self) -> style::header::Style {
        style::header::Style {
            text_color: None,
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_right: 0.0,
            offset_left: 0.0,
        }
    }

    fn hovered(&self) -> style::header::Style {
        style::header::Style {
            background: None,
            ..self.style()
        }
    }
}

impl table_row::StyleSheet for Theme {
    fn style(&self) -> style::table_row::Style {
        style::table_row::Style {
            text_color: None,
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_right: 0.0,
            offset_left: 0.0,
        }
    }

    fn hovered(&self) -> style::table_row::Style {
        style::table_row::Style {
            background: None,
            ..self.style()
        }
    }
}