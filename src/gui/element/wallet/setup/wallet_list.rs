
use crate::{gui::element::settings::wallet, log_error};
use iced::button::StyleSheet;
use iced_native::Widget;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::ColorPalette,
    grin_gui_core::{fs::PersistentData, wallet::WalletInterface},
    iced::{
        alignment, button, text_input, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Space, Text, TextInput,
    },
    //std::sync::{Arc, RwLock},
};


pub struct StateContainer {
    pub back_button_state: button::State
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            back_button_state: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
}


pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    match message {
        LocalViewInteraction::Back => {
            grin_gui.wallet_state.setup_state.mode = super::Mode::Init;
        }
    }

    Ok(Command::none())
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("wallet-list"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(style::BrightBackgroundContainer(color_palette));

    // Title row and back button
    let back_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .height(Length::Units(20))
            .align_y(alignment::Vertical::Bottom)
            .align_x(alignment::Horizontal::Center);

    let back_button: Element<Interaction> =
        Button::new(&mut state.back_button_state, back_button_label_container)
            .style(style::NormalTextButton(color_palette))
            .on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::Back,
            ))
            .into();

    let title_row = Row::new()
        .push(title_container)
        .push(Space::new(Length::Units(100), Length::Units(0)))
        .push(back_button.map(Message::Interaction))
        .align_items(Alignment::Center);

    let column = Column::new()
        .push(title_row)
        .align_items(Alignment::Center);

    Container::new(column)
        //.center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
}
