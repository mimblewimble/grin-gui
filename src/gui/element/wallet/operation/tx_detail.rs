use super::tx_list::{self, ExpandType};
use crate::log_error;
use async_std::prelude::FutureExt;
use grin_gui_core::{
    config::Config,
    error::GrinWalletInterfaceError,
    wallet::{TxLogEntry, TxLogEntryType},
};
use grin_gui_widgets::widget::header;
use iced_aw::Card;
use iced_core::Widget;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use super::tx_list::{HeaderState, TxList};

use {
    super::super::super::{
        BUTTON_HEIGHT, BUTTON_WIDTH, DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING,
        SMALLER_FONT_SIZE,
    },
    crate::gui::{GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    anyhow::Context,
    grin_gui_core::theme::{
        Button, Column, Container, Element, Header, PickList, Row, Scrollable, TableRow, Text,
        TextInput,
    },
    grin_gui_core::wallet::{InitTxArgs, Slate, StatusMessage, WalletInfo, WalletInterface},
    grin_gui_core::{
        node::{amount_from_hr_string, amount_to_hr_string},
        theme::{ButtonStyle, ColorPalette, ContainerStyle},
    },
    iced::widget::{button, pick_list, scrollable, text_input, Checkbox, Space},
    iced::{alignment, Alignment, Command, Length},
    serde::{Deserialize, Serialize},
    std::sync::{Arc, RwLock},
};

pub struct StateContainer {
    // Transaction that we're viewing
    pub current_tx: Option<TxLogEntry>,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            current_tx: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    let state = &mut grin_gui.wallet_state.operation_state.create_tx_state;

    match message {
        LocalViewInteraction::Back => {
            log::debug!("Interaction::WalletOperationTxDetailViewInteraction(Back)");
            grin_gui.wallet_state.operation_state.mode =
                crate::gui::element::wallet::operation::Mode::Home;
        }
    }

    Ok(Command::none())
}

pub fn data_container<'a>(config: &'a Config, state: &'a StateContainer) -> Container<'a, Message> {
    // Title row
    let title = Text::new(localized_string("tx-details-title"))
        .size(DEFAULT_HEADER_FONT_SIZE)
        .horizontal_alignment(alignment::Horizontal::Center);

    let title_container = Container::new(title)
        .style(grin_gui_core::theme::ContainerStyle::BrightBackground)
        .padding(iced::Padding::from([
            2, // top
            0, // right
            2, // bottom
            5, // left
        ]));

    let header_row = Row::new().push(title_container);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,               // top
        0,               // right
        DEFAULT_PADDING as u16, // bottom
        0,               // left
    ]));

    let unit_spacing = 15.0;
    let row_spacing = 5.0;

    let button_height = Length::Fixed(BUTTON_HEIGHT);
    let button_width = Length::Fixed(BUTTON_WIDTH);

    let mut column = Column::new();

    if let Some(ref tx) = state.current_tx {
        // ID
        let id_label = Text::new(format!("{}:  ", localized_string("tx-id")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let id_label_container =
            Container::new(id_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let id_value = Text::new(format!("{}", tx.id))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let id_value_container =
            Container::new(id_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let id_row = Row::new().push(id_label_container).push(id_value_container);

        column = column
            .push(id_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Tx Type
        let tx_type_label = Text::new(format!("{}:  ", localized_string("tx-type")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_type_label_container =
            Container::new(tx_type_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_type_value = Text::new(format!("{}", tx.tx_type))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_type_value_container =
            Container::new(tx_type_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_type_row = Row::new().push(tx_type_label_container).push(tx_type_value_container);

        column = column
            .push(tx_type_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));


        // UUID
        let shared_tx_id_label = Text::new(format!("{}:  ", localized_string("tx-shared-id")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let shared_tx_id_label_container =
            Container::new(shared_tx_id_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let uuid = match tx.tx_slate_id {
            Some(u) => u.to_string(),
            None => "None".to_owned(),
        };

        let shared_tx_id_value = Text::new(format!("{}", uuid))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let shared_tx_id_value_container =
            Container::new(shared_tx_id_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let shared_tx_id_row = Row::new().push(shared_tx_id_label_container).push(shared_tx_id_value_container);

        column = column
            .push(shared_tx_id_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Creation Time
        let tx_creation_time_label = Text::new(format!("{}:  ", localized_string("tx-creation-time")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_creation_time_label_container =
            Container::new(tx_creation_time_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_creation_time_value = Text::new(format!("{}", tx.creation_ts))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_creation_time_value_container =
            Container::new(tx_creation_time_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_creation_time_row = Row::new().push(tx_creation_time_label_container).push(tx_creation_time_value_container);

        column = column
            .push(tx_creation_time_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // TTL Cutoff Height
        let ttl_cutoff_label = Text::new(format!("{}:  ", localized_string("tx-ttl-cutoff")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let ttl_cutoff_label_container =
            Container::new(ttl_cutoff_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let ttl = match tx.ttl_cutoff_height {
            Some(u) => u.to_string(),
            None => "None".to_owned(),
        };

        let ttl_cutoff_value = Text::new(format!("{}", ttl))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let ttl_cutoff_value_container =
            Container::new(ttl_cutoff_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let ttl_cutoff_row = Row::new().push(ttl_cutoff_label_container).push(ttl_cutoff_value_container);

        column = column
            .push(ttl_cutoff_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Confirmed
        let confirmed_label = Text::new(format!("{}:  ", localized_string("tx-is-confirmed")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let confirmed_label_container =
            Container::new(confirmed_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let confirmed_value = Text::new(format!("{}", tx.confirmed))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let confirmed_value_container =
            Container::new(confirmed_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let confirmed_row = Row::new().push(confirmed_label_container).push(confirmed_value_container);

        column = column
            .push(confirmed_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Confirmation Time
        let tx_confirmation_time_label = Text::new(format!("{}:  ", localized_string("tx-confirmation-time")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_confirmation_time_label_container =
            Container::new(tx_confirmation_time_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let time = match tx.confirmation_ts {
            Some(u) => u.to_string(),
            None => "None".to_owned(),
        };

        let tx_confirmation_time_value = Text::new(format!("{}", time))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_confirmation_time_value_container =
            Container::new(tx_confirmation_time_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_confirmation_time_row = Row::new().push(tx_confirmation_time_label_container).push(tx_confirmation_time_value_container);
        column = column
            .push(tx_confirmation_time_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Number of Inputs
        let tx_num_inputs_label = Text::new(format!("{}:  ", localized_string("tx-num-inputs")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_num_inputs_label_container =
            Container::new(tx_num_inputs_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_num_inputs_value = Text::new(format!("{}", tx.num_inputs))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_num_inputs_value_container =
            Container::new(tx_num_inputs_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_num_inputs_row = Row::new().push(tx_num_inputs_label_container).push(tx_num_inputs_value_container);
        column = column
            .push(tx_num_inputs_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Number of Outputs
        let tx_num_outputs_label = Text::new(format!("{}:  ", localized_string("tx-num-outputs")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_num_outputs_label_container =
            Container::new(tx_num_outputs_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_num_outputs_value = Text::new(format!("{}", tx.num_outputs))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_num_outputs_value_container =
            Container::new(tx_num_outputs_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_num_outputs_row = Row::new().push(tx_num_outputs_label_container).push(tx_num_outputs_value_container);
        column = column
            .push(tx_num_outputs_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Amount Credited
        let tx_amount_credited_label = Text::new(format!("{}:  ", localized_string("tx-amount-credited")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_amount_credited_label_container =
            Container::new(tx_amount_credited_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_amount_credited_value = Text::new(format!("{}", amount_to_hr_string(tx.amount_credited, true)))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_amount_credited_value_container =
            Container::new(tx_amount_credited_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_amount_credited_row = Row::new().push(tx_amount_credited_label_container).push(tx_amount_credited_value_container);
        column = column
            .push(tx_amount_credited_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Amount Debited
        let tx_amount_debited_label = Text::new(format!("{}:  ", localized_string("tx-amount-debited")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_amount_debited_label_container =
            Container::new(tx_amount_debited_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_amount_debited_value = Text::new(format!("{}", amount_to_hr_string(tx.amount_debited, true)))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_amount_debited_value_container =
            Container::new(tx_amount_debited_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_amount_debited_row = Row::new().push(tx_amount_debited_label_container).push(tx_amount_debited_value_container);
        column = column
            .push(tx_amount_debited_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Fee
        let tx_fee_label = Text::new(format!("{}:  ", localized_string("tx-fee")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_fee_label_container =
            Container::new(tx_fee_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let fee = match tx.fee {
            Some(u) => format!("{}", amount_to_hr_string(u.fee(), true)),
            None => "None".to_owned(),
        };

        let tx_fee_value = Text::new(format!("{}", fee))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_fee_value_container =
            Container::new(tx_fee_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_fee_row = Row::new().push(tx_fee_label_container).push(tx_fee_value_container);
        column = column
            .push(tx_fee_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(row_spacing)));

        // Net Difference
        let tx_net_difference_label = Text::new(format!("{}:  ", localized_string("tx-net-difference")))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_net_difference_label_container =
            Container::new(tx_net_difference_label).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		let net_diff = if tx.amount_credited >= tx.amount_debited {
			amount_to_hr_string(tx.amount_credited - tx.amount_debited, true)
		} else {
			format!(
				"-{}",
				amount_to_hr_string(tx.amount_debited - tx.amount_credited, true)
			)
		};
	
        let tx_net_difference_value = Text::new(format!("{}", net_diff))
            .size(DEFAULT_FONT_SIZE)
            .horizontal_alignment(alignment::Horizontal::Left);

        let tx_net_difference_value_container =
            Container::new(tx_net_difference_value).style(grin_gui_core::theme::ContainerStyle::NormalBackground);

        let tx_net_difference_row = Row::new().push(tx_net_difference_label_container).push(tx_net_difference_value_container);
        column = column
            .push(tx_net_difference_row)
            .push(Space::new(Length::Fixed(0.0), Length::Fixed(unit_spacing)));

    }

    let cancel_button_label_container =
        Container::new(Text::new(localized_string("back")).size(DEFAULT_FONT_SIZE))
            .width(button_width)
            .height(button_height)
            .center_x()
            .center_y()
            .align_x(alignment::Horizontal::Center);

    let cancel_button: Element<Interaction> = Button::new(cancel_button_label_container)
        .style(grin_gui_core::theme::ButtonStyle::Primary)
        .on_press(Interaction::WalletOperationTxDetailViewInteraction(
            LocalViewInteraction::Back,
        ))
        .into();

    let cancel_container = Container::new(cancel_button.map(Message::Interaction)).padding(1);
    let cancel_container = Container::new(cancel_container)
        .style(grin_gui_core::theme::ContainerStyle::Segmented)
        .padding(1);

    let button_row = Row::new()
        .push(cancel_container)
        .push(Space::new(Length::Fixed(unit_spacing), Length::Fixed(0.0)));

    column = column.push(button_row);

    let form_container = Container::new(column)
        .width(Length::Fill)
        .padding(iced::Padding::from([
            0, // top
            0, // right
            0, // bottom
            5, // left
        ]));

    // form container should be scrollable in tiny windows
    let scrollable = Scrollable::new(form_container)
        .height(Length::Fill)
        .style(grin_gui_core::theme::ScrollableStyle::Primary);

    let content = Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(grin_gui_core::theme::ContainerStyle::NormalBackground);

    let wrapper_column = Column::new()
        .height(Length::Fill)
        .push(header_container)
        .push(content);

    // Returns the final container.
    Container::new(wrapper_column).padding(iced::Padding::from([
        DEFAULT_PADDING, // top
        DEFAULT_PADDING, // right
        DEFAULT_PADDING, // bottom
        DEFAULT_PADDING, // left
    ]))
}
