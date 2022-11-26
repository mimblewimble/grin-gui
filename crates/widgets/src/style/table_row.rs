//! Decorate content and apply alignment.
use iced_core::{Background, Color};

/// The appearance of a table row.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub offset_left: f32,
    pub offset_right: f32,
}

/// A set of rules that dictate the style of a table row.
pub trait StyleSheet {
    type Style: std::default::Default + Copy;

    fn style(&self) -> Style;

    /// Produces the style of a hovered table row.
    fn hovered(&self) -> Style;
}

// add default impl for Stylesheet
impl StyleSheet for () {
    type Style = Style;

    fn style(&self) -> Style {
        Style::default()
    }

    fn hovered(&self) -> Style {
        Style::default()
    }
}

// impl default for style sheet
// impl<T> StyleSheet for T
// where
//     T: std::default::Default + Copy,
// {
//     type Style = T;

//     fn style(&self) -> Style {
//         Style::default()
//     }

//     fn hovered(&self) -> Style {
//         Style::default()
//     }
// }

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            text_color: None,
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
}

// impl<'a> std::default::Default for Box<dyn StyleSheet + 'a> {
//     fn default() -> Self {
//         Box::new(Default)
//     }
// }

// impl<'a> From<T> for Box<dyn StyleSheet + 'a>
// where
//     T: 'a + StyleSheet,
// {
//     fn from(style: T) -> Self {
//         Box::new(style)
//     }
// }
