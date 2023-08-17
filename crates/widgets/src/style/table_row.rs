use iced::{Background, Color};

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
    type Style: std::default::Default + Copy;

    /// Produces the default appearance of a table row.
    fn appearance(&self, style: &Self::Style) -> Appearance;

    /// Produces the hovered appearance table row.
    fn hovered(&self, style: &Self::Style) -> Appearance;
}
