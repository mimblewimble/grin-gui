use iced_core::{Background, Color};

/// The appearance of a header.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub offset_left: f32,
    pub offset_right: f32,
}

/// A set of rules that dictate the style of a header.
pub trait StyleSheet {
    type Style: std::default::Default + Copy;

    /// Produces the style of a header.
    fn appearance(&self, style: &Self::Style) -> Appearance;

    /// Produces the a hovered appearance for header.
    fn hovered(&self, style: &Self::Style) -> Appearance;
  
}