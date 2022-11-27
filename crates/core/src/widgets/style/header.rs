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

// impl StyleSheet for crate::theme::Theme {
//     fn style(&self) -> Style {
//         Style {
//             text_color: None,
//             background: None,
//             border_radius: 0.0,
//             border_width: 0.0,
//             border_color: Color::TRANSPARENT,
//             offset_right: 0.0,
//             offset_left: 0.0,
//         }
//     }

//     fn hovered(&self) -> Style {
//         Style {
//             background: None,
//             ..self.style()
//         }
//     }
// }

// pub struct Default;
// impl StyleSheet for Default {
//     fn style(&self) -> Style {
//         Style {
//             text_color: None,
//             background: None,
//             border_radius: 0.0,
//             border_width: 0.0,
//             border_color: Color::TRANSPARENT,
//             offset_right: 0.0,
//             offset_left: 0.0,
//         }
//     }

//     fn hovered(&self) -> Style {
//         Style {
//             background: None,
//             ..self.style()
//         }
//     }
// }


// impl<'a> std::default::Default for Box<dyn StyleSheet + 'a> {
//     fn default() -> Self {
//         Box::new(Default)
//     }
// }

// impl<'a, T> From<T> for Box<dyn StyleSheet + 'a>
// where
//     T: 'a + StyleSheet,
// {
//     fn from(style: T) -> Self {
//         Box::new(style)
//     }
// }
