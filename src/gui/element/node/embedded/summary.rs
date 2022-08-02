use crate::{gui::element::settings::wallet, log_error};
use iced::button::StyleSheet;
use iced_native::Widget;
use std::path::PathBuf;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{
        fs::PersistentData,
        wallet::{StatusMessage, WalletInfo, WalletInterface},
    },
    iced::{
        alignment, button, text_input, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text, TextInput,
    },
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let stats = &grin_gui.node_state.embedded_state.server_stats;
    let state = &mut grin_gui.node_state.embedded_state.summary_state;
    /*match message {
    }*/
    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("node-summary-home"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container =
        Container::new(title).style(style::BrightBackgroundContainer(color_palette));

    let title_row = Row::new()
        .push(title_container)
        .align_items(Alignment::Center)
        .padding(6)
        .spacing(20);

    // Basic Info "Box"

    let colum = Column::new()
        .push(title_row)
        .push(Space::new(Length::Units(0), Length::Fill))
        .align_items(Alignment::Center);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
