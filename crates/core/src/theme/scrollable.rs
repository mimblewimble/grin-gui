use iced::widget::scrollable;
use iced::{Background, Color};
use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum ScrollableStyles {
    #[default]
    Default,
    Primary,
}

impl scrollable::StyleSheet for Theme {
    type Style = ScrollableStyles;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            ScrollableStyles::Default => scrollable::Scrollbar {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: Color::TRANSPARENT,
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },

            ScrollableStyles::Primary =>  scrollable::Scrollbar {
                background: Some(Background::Color(self.palette.base.background)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: self.palette.base.foreground,
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let active = self.active(style);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..active.scroller },
            ..active
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}

