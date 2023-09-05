use {
    super::{DEFAULT_FONT_SIZE},
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::config::{Config, TxMethod},
    grin_gui_core::theme::{Button, Column, Container, PickList, Row, Scrollable, Text, TextInput},
    grin_gui_core::fs::PersistentData,
    iced::Length,
    iced::widget::{
        button, pick_list, scrollable, text_input, Checkbox, Space,
    },
    iced::Alignment,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone)]
pub struct StateContainer {
    // scrollable_state: scrollable::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalViewInteraction {
    TxMethodSelected(TxMethod),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    Wallet,
    Node,
    General,
}

pub fn handle_message(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) {
    match message {
        LocalViewInteraction::TxMethodSelected(method) => {
            log::debug!("Interaction::TxMethodSelectedSettings({:?})", method);
            // Set Mode
            grin_gui.config.tx_method = method;
            let _ = grin_gui.config.save();
        }
    }
}

pub fn data_container<'a>(
    state: &'a StateContainer,
    config: &Config,
) -> Container<'a, Message> {
   
    let tx_method_column = {
        let tx_method_container =
            Container::new(Text::new(localized_string("tx-method")).size(DEFAULT_FONT_SIZE))
                .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_method_pick_list = PickList::new(
            &TxMethod::ALL[..],
            Some(config.tx_method),
            |t| {
                Message::Interaction(Interaction::WalletSettingsViewInteraction(
                    LocalViewInteraction::TxMethodSelected(t),
                ))
            },
        )
        .text_size(DEFAULT_FONT_SIZE)
        .width(Length::Fixed(120.0))
        .style(grin_gui_core::theme::PickListStyle::Primary);

        // Data row for theme picker list.
        let tx_method_data_row = Row::new()
            .push(tx_method_pick_list)
            .align_items(Alignment::Center)
            .height(Length::Fixed(26.0));

        Column::new()
            .push(tx_method_container)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(5.0)))
            .push(tx_method_data_row)
    };

    let wrap = {
        Column::new()
            .push(tx_method_column)
    };

    let scrollable = Scrollable::new(wrap)
    .height(Length::Fill)
    .style(grin_gui_core::theme::ScrollableStyle::Primary);

    let col = Column::new()
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(10.0)))
        .push(scrollable)
        .push(Space::new(Length::Fixed(0.0), Length::Fixed(20.0)));
    let row = Row::new()
        .push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
        .push(col);
 
    // Returns the final container.
    Container::new(row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground)
}
