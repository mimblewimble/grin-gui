//! Decorate content and apply alignment.
use iced_core::{Background, Color};
use crate::theme::Theme;

/// The appearance of a table row.
#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
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
    fn style(&self) -> Appearance;

    /// Produces the style of a hovered table row.
    fn hovered(&self) -> Appearance;
}


impl StyleSheet for Theme {
    fn style(&self) -> Appearance {
        Appearance::default()
    }

    fn hovered(&self) -> Appearance {
        Appearance::default()
    }
}


pub struct Default;
impl StyleSheet for Default {
    fn style(&self) -> Appearance {
        Appearance::default()
    }

    fn hovered(&self) -> Appearance {
        Appearance::default()
    }
}


impl<'a> std::default::Default for Box<dyn StyleSheet + 'a> {
    fn default() -> Self {
        Box::new(Default)
    }
}


impl<'a, T> From<T> for Box<dyn StyleSheet + 'a>
where
    T: 'a + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}


/// A set of rules that dictate the style of a table row with assoicated style type.
pub trait StyleSheetAssociated {
    type Style: std::default::Default + Clone;

    fn appearance(&self, style: Self::Style) -> Appearance;

    /// Produces the style of a hovered table row.
    fn hovered(&self, style: Self::Style) -> Appearance;
}
