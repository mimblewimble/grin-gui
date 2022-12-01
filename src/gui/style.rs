use grin_gui_core::theme::ColorPalette;
use grin_gui_widgets::{table_row, style};
use iced::widget::{button, checkbox, container, scrollable, slider, text_input};
use iced::{Background, Color};
use iced_aw::style::{ModalStyles, CardStyles};
use iced_aw::style::modal::Appearance;
use iced_aw::{card, modal};

pub struct NormalModalContainer(pub ColorPalette);
impl modal::StyleSheet for NormalModalContainer {
    type Style = ModalStyles;

    fn active(&self, style: Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color {
                a: 0.9,
                ..self.0.base.foreground
            }),
        }
    }
}

pub struct NormalModalCardContainer(pub ColorPalette);
impl card::StyleSheet for NormalModalCardContainer {
    type Style = CardStyles;

    fn active(&self, style: Self::Style) -> iced_aw::style::card::Appearance {
        iced_aw::style::card::Appearance {
            background: Background::Color(self.0.base.background),
            head_background: Background::Color(self.0.normal.primary),
            head_text_color: self.0.bright.surface,
            border_color: self.0.normal.primary,
            body_text_color: self.0.normal.surface,
            border_radius: 5.0,
            ..card::Appearance::default()
        }
    }
}



pub struct BrightForegroundContainer(pub ColorPalette);
impl container::StyleSheet for BrightForegroundContainer {
    type Style = iced_style::theme::Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.bright.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct NormalForegroundContainer(pub ColorPalette);
impl container::StyleSheet for NormalForegroundContainer {
    type Style = iced_style::theme::Container;
   
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct HoverableBrightForegroundContainer(pub ColorPalette);
impl container::StyleSheet for HoverableBrightForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: None,
    //         text_color: Some(self.0.bright.primary),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct HoverableForegroundContainer(pub ColorPalette);
impl container::StyleSheet for HoverableForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: None,
    //         text_color: Some(self.0.normal.surface),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct HoverableSegmentContainer(pub ColorPalette);
impl container::StyleSheet for HoverableSegmentContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: None,
    //         text_color: Some(self.0.bright.primary),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct HoverableSegmentAlternateContainer(pub ColorPalette);
impl container::StyleSheet for HoverableSegmentAlternateContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(self.0.base.foreground)),
    //         text_color: Some(self.0.bright.primary),
    //         border_radius: 15.0,
    //         border_width: 1.0,
    //         border_color: Color {
    //             a: 1.0,
    //             ..self.0.normal.primary
    //         },
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct FadedNormalForegroundContainer(pub ColorPalette);
impl container::StyleSheet for FadedNormalForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(Color {
    //             a: 0.80,
    //             ..self.0.base.foreground
    //         })),

    //         text_color: Some(self.0.normal.surface),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct SelectedBrightForegroundContainer(pub ColorPalette);
impl container::StyleSheet for SelectedBrightForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: None,
    //         text_color: Some(self.0.bright.primary),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct FadedBrightForegroundContainer(pub ColorPalette);
impl container::StyleSheet for FadedBrightForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         text_color: Some(self.0.bright.surface),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct NormalBackgroundContainer(pub ColorPalette);
impl container::StyleSheet for NormalBackgroundContainer {
    type Style = iced_style::theme::Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct BrightBackgroundContainer(pub ColorPalette);
impl container::StyleSheet for BrightBackgroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(self.0.base.background)),
    //         text_color: Some(self.0.bright.surface),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct SegmentedContainer(pub ColorPalette);
impl container::StyleSheet for SegmentedContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         border_radius: 2.0,
    //         border_width: 1.0,
    //         border_color: Color {
    //             a: 0.5,
    //             ..self.0.normal.primary
    //         },
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct NormalErrorBackgroundContainer(pub ColorPalette);
impl container::StyleSheet for NormalErrorBackgroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(self.0.base.background)),
    //         text_color: Some(self.0.normal.error),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct NormalSuccessBackgroundContainer(pub ColorPalette);
impl container::StyleSheet for NormalSuccessBackgroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(self.0.base.background)),
    //         text_color: Some(self.0.normal.secondary),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct NormalErrorForegroundContainer(pub ColorPalette);
impl container::StyleSheet for NormalErrorForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(self.0.base.foreground)),
    //         text_color: Some(self.0.normal.error),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}
pub struct NormalSuccessForegroundContainer(pub ColorPalette);
impl container::StyleSheet for NormalSuccessForegroundContainer {
    type Style = iced_style::theme::Container;
    // fn style(&self) -> container::Style {
    //     container::Style {
    //         background: Some(Background::Color(self.0.base.foreground)),
    //         text_color: Some(self.0.normal.secondary),
    //         ..container::Style::default()
    //     }
    // }
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.normal.surface),
            ..container::Appearance::default()
        }
    }
}

pub struct BrightTextButton(pub ColorPalette);
impl button::StyleSheet for BrightTextButton {
    type Style = iced_style::theme::Button;
    
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: self.0.bright.surface,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }
}

pub struct NormalTextButton(pub ColorPalette);
impl button::StyleSheet for NormalTextButton {
    type Style = iced_style::theme::Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: self.0.normal.surface,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }
}

pub struct SelectedBrightTextButton(pub ColorPalette);
impl button::StyleSheet for SelectedBrightTextButton {
    type Style = iced_style::theme::Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: self.0.bright.primary,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }
}

pub struct DefaultButton(pub ColorPalette);
impl button::StyleSheet for DefaultButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: self.0.bright.primary,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            })),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color {
                a: 0.25,
                ..self.0.normal.surface
            },
            ..self.active(style)
        }
    }
}

pub struct DefaultBoxedButton(pub ColorPalette);
impl button::StyleSheet for DefaultBoxedButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
            border_width: 1.0,
            border_radius: 2.0,
            text_color: self.0.bright.primary,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            })),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.05,
                ..self.0.normal.primary
            })),
            text_color: Color {
                a: 0.50,
                ..self.0.normal.primary
            },
            ..self.active(style)
        }
    }
}

pub struct SecondaryBoxedButton(pub ColorPalette);
impl button::StyleSheet for SecondaryBoxedButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.15,
                ..self.0.normal.secondary
            })),
            text_color: self.0.bright.secondary,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.normal.secondary)),
            text_color: self.0.bright.secondary,
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.05,
                ..self.0.normal.secondary
            })),
            text_color: Color {
                a: 0.15,
                ..self.0.bright.secondary
            },
            ..self.active(style)
        }
    }
}

pub struct SecondaryButton(pub ColorPalette);
impl button::StyleSheet for SecondaryButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: self.0.bright.secondary,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.normal.secondary)),
            text_color: self.0.bright.secondary,
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color {
                a: 0.25,
                ..self.0.normal.surface
            },
            ..self.active(style)
        }
    }
}

pub struct DefaultDeleteButton(pub ColorPalette);
impl button::StyleSheet for DefaultDeleteButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: 2.0,
            text_color: self.0.bright.error,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.error
            })),
            text_color: self.0.bright.error,
            ..self.active(style)
        }
    }
}

pub struct ColumnHeaderButton(pub ColorPalette);
impl button::StyleSheet for ColumnHeaderButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.base.background)),
            text_color: Color {
                ..self.0.bright.surface
            },
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.15,
                ..self.0.normal.primary
            })),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }
}

pub struct UnclickableColumnHeaderButton(pub ColorPalette);
impl button::StyleSheet for UnclickableColumnHeaderButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        ColumnHeaderButton(self.0).active(style)
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

pub struct SelectedColumnHeaderButton(pub ColorPalette);
impl button::StyleSheet for SelectedColumnHeaderButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.base.background)),
            text_color: Color {
                ..self.0.bright.primary
            },
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            })),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }
}

pub struct DisabledDefaultButton(pub ColorPalette);
impl button::StyleSheet for DisabledDefaultButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color {
                a: 0.25,
                ..self.0.normal.surface
            },
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}

pub struct SelectedDefaultButton(pub ColorPalette);
impl button::StyleSheet for SelectedDefaultButton {
    type Style = iced_style::theme::Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.normal.primary)),
            text_color: self.0.bright.primary,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.normal.primary)),
            text_color: self.0.bright.primary,
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}

pub struct Row(pub ColorPalette);
impl container::StyleSheet for Row {
    type Style = iced_style::theme::Container;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.background)),
            ..container::Appearance::default()
        }
    }
}

pub struct TableRow(pub ColorPalette);
impl table_row::StyleSheet for TableRow {
    fn style(&self) -> table_row::Style {
        table_row::Style {
            text_color: None,
            background: Some(Background::Color(self.0.base.foreground)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 10.0,
            offset_right: 25.0,
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct TableRowHighlife(pub ColorPalette);
impl table_row::StyleSheet for TableRowHighlife {
    fn style(&self) -> table_row::Style {
        table_row::Style {
            text_color: None,
            background: Some(Background::Color(Color {
                a: 0.30,
                ..self.0.base.foreground
            })),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct TableRowLowlife(pub ColorPalette);
impl table_row::StyleSheet for TableRowLowlife {
    fn style(&self) -> table_row::Style {
        table_row::Style {
            text_color: None,
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct TableRowSelected(pub ColorPalette);
impl table_row::StyleSheet for TableRowSelected {
    fn style(&self) -> table_row::Style {
        table_row::Style {
            text_color: None,
            background: Some(Background::Color(self.0.normal.primary)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.60,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

/*pub struct SegmentTableRow(pub ColorPalette);
impl table_row::StyleSheet for SegmentTableRow {
    fn style(&self) -> table_row::Style {
        table_row::Style {
            text_color: None,
            background: Some(Background::Color(self.0.base.foreground)),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.15,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct SelectedSegmentTableRow(pub ColorPalette);
impl table_row::StyleSheet for SelectedSegmentTableRow {
    fn style(&self) -> table_row::Style {
        table_row::Style {
            text_color: None,
            background: Some(Background::Color(self.0.normal.primary)),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style { ..style }
    }
}*/

pub struct TableRowAlternate(pub ColorPalette);
impl table_row::StyleSheet for TableRowAlternate {
    fn style(&self) -> table_row::Style {
        let default = TableRow(self.0).style();

        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.50,
                ..self.0.base.foreground
            })),
            ..default
        }
    }
    fn hovered(&self) -> table_row::Style {
        let style = self.style();
        table_row::Style {
            background: Some(Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            })),
            ..style
        }
    }
}

pub struct ForegroundScrollable(pub ColorPalette);
impl scrollable::StyleSheet for ForegroundScrollable {
    type Style = iced_style::theme::Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(self.0.base.foreground)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.0.base.background,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
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

pub struct Scrollable(pub ColorPalette);
impl scrollable::StyleSheet for Scrollable {
    type Style = iced_style::theme::Scrollable;
    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(self.0.base.background)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.0.base.foreground,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
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

pub struct PickList(pub ColorPalette);
impl iced::widget::pick_list::StyleSheet for PickList {
    type Style = iced_style::theme::PickList;

    // fn menu(&self) -> t::Menu {
    //     pick_list::Menu {
    //         text_color: self.0.bright.surface,
    //         background: Background::Color(self.0.base.foreground),
    //         border_width: 1.0,
    //         border_color: self.0.base.background,
    //         selected_background: Background::Color(Color {
    //             a: 0.15,
    //             ..self.0.normal.primary
    //         }),
    //         selected_text_color: self.0.bright.primary,
    //     }
    // }

    fn active(&self, style: &Self::Style) -> iced::widget::pick_list::Appearance {
        iced::widget::pick_list::Appearance {
            text_color: self.0.bright.surface,
            background: self.0.base.background.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::pick_list::Appearance { 
        let active = self.active(style);
        iced::widget::pick_list::Appearance {
            text_color: self.0.bright.primary,
            ..active
        }
    }
}

pub struct SecondaryPickList(pub ColorPalette);
impl iced::widget::pick_list::StyleSheet for SecondaryPickList {
    type Style = iced_style::theme::PickList;
    // fn menu(&self) -> pick_list::Menu {
    //     pick_list::Menu {
    //         text_color: self.0.bright.surface,
    //         background: Background::Color(self.0.base.background),
    //         border_width: 1.0,
    //         border_color: self.0.base.foreground,
    //         selected_background: Background::Color(Color {
    //             a: 0.15,
    //             ..self.0.normal.primary
    //         }),
    //         selected_text_color: self.0.bright.primary,
    //     }
    // }

    fn active(&self, style: &Self::Style) -> iced::widget::pick_list::Appearance {
        iced::widget::pick_list::Appearance {
            text_color: self.0.bright.surface,
            background: self.0.base.foreground.into(),
            border_width: 0.0,
            border_color: self.0.base.background,
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::pick_list::Appearance { 
        let active = self.active(style);
        iced::widget::pick_list::Appearance {
            background: Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            }),
            text_color: self.0.bright.primary,
            ..active
        }
    }
}

/*pub struct MenuPickList(pub ColorPalette);
impl pick_list::StyleSheet for MenuPickList {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            text_color: self.0.bright.primary,
            background: Background::Color(self.0.base.background),
            selected_background: Background::Color(Color {
                a: 0.15,
                ..self.0.normal.primary
            }),
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
            selected_text_color: self.0.bright.primary,
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: self.0.bright.primary,
            background: self.0.base.foreground.into(),
            border_width: 0.0,
            border_radius: 2.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        let active = self.active();
        pick_list::Style {
            background: Background::Color(Color {
                a: 0.25,
                ..self.0.normal.primary
            }),
            text_color: self.0.bright.primary,
            ..active
        }
    }
}*/

pub struct PanelBordered(pub ColorPalette);
impl container::StyleSheet for PanelBordered {
    type Style = iced_style::theme::Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Some(self.0.bright.primary),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }
}

pub struct PanelForeground(pub ColorPalette);
impl container::StyleSheet for PanelForeground {
    type Style = iced_style::theme::Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.bright.primary),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

pub struct ChannelBadge(pub ColorPalette);
impl container::StyleSheet for ChannelBadge {
    type Style = iced_style::theme::Container;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.bright.primary),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }
}

pub struct ChannelBadgeFaded(pub ColorPalette);
impl container::StyleSheet for ChannelBadgeFaded {
    type Style = iced_style::theme::Container;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.base.foreground)),
            text_color: Some(self.0.bright.primary),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }
}

pub struct DefaultCheckbox(pub ColorPalette);
impl checkbox::StyleSheet for DefaultCheckbox {
    type Style = iced_style::theme::Checkbox;
    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(self.0.base.background),
            checkmark_color: self.0.bright.primary,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.0.normal.primary,
            text_color: Some(self.0.normal.surface),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(self.0.base.foreground),
            checkmark_color: self.0.bright.primary,
            border_radius: 2.0,
            border_width: 2.0,
            border_color: self.0.bright.primary,
            text_color: Some(self.0.normal.surface),
        }
    }
}

/*pub struct AlwaysCheckedCheckbox(pub ColorPalette);
impl checkbox::StyleSheet for AlwaysCheckedCheckbox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(self.0.base.background),
            checkmark_color: self.0.normal.primary,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.0.normal.primary,
        }
    }

    fn hovered(&self, _is_checked: bool) -> checkbox::Style {
        self.active(_is_checked)
    }
}*/

pub struct CatalogQueryInput(pub ColorPalette);
impl text_input::StyleSheet for CatalogQueryInput {
    type Style = iced_style::theme::TextInput;
    /// Produces the style of an active text input.
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.0.base.foreground),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: self.0.base.foreground,
        }
    }

    /// Produces the style of a focused text input.
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.0.base.foreground),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        self.0.normal.surface
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        self.0.bright.primary
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        self.0.bright.secondary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

pub struct Slider(pub ColorPalette);
impl slider::StyleSheet for Slider {
    type Style = iced_style::theme::Slider;

    fn active(&self, style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (self.0.base.foreground, self.0.base.foreground),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 9.0 },
                color: self.0.bright.primary,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        let active = self.active(style);

        slider::Appearance {
            handle: slider::Handle {
                color: self.0.bright.primary,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        let active = self.active(style);

        slider::Appearance {
            handle: slider::Handle {
                color: self.0.bright.primary,
                ..active.handle
            },
            ..active
        }
    }
}

pub struct AddonsQueryInput(pub ColorPalette);
impl text_input::StyleSheet for AddonsQueryInput {
    type Style = iced_style::theme::TextInput;

    /// Produces the style of an active text input.
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.0.base.foreground),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: self.0.base.foreground,
        }
    }

    /// Produces the style of a focused text input.
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.0.base.foreground),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.0.normal.primary
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        self.0.normal.surface
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        self.0.bright.primary
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        self.0.bright.secondary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}
