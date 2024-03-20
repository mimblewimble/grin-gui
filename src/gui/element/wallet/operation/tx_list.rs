use std::borrow::Borrow;

use grin_gui_core::config::TxMethod;
use iced_core::Widget;

use {
	super::super::super::{BUTTON_WIDTH, DEFAULT_FONT_SIZE, DEFAULT_PADDING, SMALLER_FONT_SIZE},
	crate::gui::{GrinGui, Interaction, Message},
	crate::localization::localized_string,
	crate::Result,
	grin_gui_core::theme::{
		Button, Column, Container, Element, Header, PickList, Row, Scrollable, TableRow, Text,
		TextInput, Theme,
	},
	grin_gui_core::widget::header,
	grin_gui_core::{
		config::Config,
		node::amount_to_hr_string,
		theme::{ButtonStyle, ColorPalette, ContainerStyle},
		wallet::{TxLogEntry, TxLogEntryType},
	},
	iced::widget::{button, pick_list, scrollable, text_input, Space},
	iced::{alignment, Alignment, Command, Length},
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
	strfmt::strfmt,
};

#[derive(Debug, Clone)]
pub enum ExpandType {
	Details(TxLogEntryWrap),
	None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Confirm {
	DeleteAddon,
	DeleteSavedVariables,
}

/*
For reference:

Transaction Log - Account 'default' - Block Height: 1926614
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 Id  Type         Shared Transaction Id  Creation Time        TTL Cutoff Height  Confirmed?  Confirmation Time    Num.    Num.     Amount          Amount   Fee        Net             Payment   Kernel  Tx
																												  Inputs  Outputs  Credited        Debited             Difference      Proof             Data
=========================================================================================================================================================================================================
 0   Received Tx  None                   2021-03-31 08:41:34  None               true        2021-03-31 08:41:34  0       1        x.x             0.0      None  x.x  None      None    None
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 1   Received Tx  None                   2021-03-31 08:41:34  None               true        2021-03-31 08:41:34  0       1        x.x             0.0      None  x.x  None      None    None
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 2   Received Tx  None                   2021-03-31 08:41:34  None               true        2021-03-31 08:41:34  0       1        x.x             0.0      None  x.x  None      None    None
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

*/

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum ColumnKey {
	Id,
	Type,
	SharedTransactionId,
	CreationTime,
	Status,
	TTLCutoff,
	Height,
	IsConfirmed,
	ConfirmationTime,
	NumInputs,
	NumOutputs,
	AmountCredited,
	AmountDebited,
	Fee,
	NetDifference,
	PaymentProof,
	Kernel,
	TxData,
	// Only used for sorting, not an actual visible column that can be shown
	FuzzyScore,
}

impl ColumnKey {
	fn title(self) -> String {
		use ColumnKey::*;

		match self {
			Id => localized_string("tx_id"),
			Type => localized_string("tx_type"),
			SharedTransactionId => localized_string("tx_shared_id"),
			CreationTime => localized_string("tx-creation-time"),
			Status => localized_string("tx-status"),
			TTLCutoff => localized_string("tx_ttl_cutoff"),
			Height => localized_string("tx_height"),
			IsConfirmed => localized_string("tx_is_confirmed"),
			ConfirmationTime => localized_string("tx-confirmation-time"),
			NumInputs => localized_string("tx_num_inputs"),
			NumOutputs => localized_string("tx_num_outputs"),
			AmountCredited => localized_string("tx_amount_credited"),
			AmountDebited => localized_string("tx_amount_debited"),
			Fee => localized_string("tx_fee"),
			NetDifference => localized_string("tx-net-difference"),
			PaymentProof => localized_string("tx_payment_proof"),
			Kernel => localized_string("tx_kernel"),
			TxData => localized_string("tx_data"),
			FuzzyScore => unreachable!("fuzzy score not used as an actual column"),
		}
	}

	fn as_string(self) -> String {
		use ColumnKey::*;

		let s = match self {
			Id => "tx_id",
			Type => "tx_type",
			SharedTransactionId => "tx_shared_id",
			CreationTime => "tx-creation-time",
			Status => "tx-status",
			TTLCutoff => "tx_ttl_cutoff",
			Height => "tx_height",
			IsConfirmed => "tx_is_confirmed",
			ConfirmationTime => "tx-confirmation-time",
			NumInputs => "tx_num_inputs",
			NumOutputs => "tx_num_outputs",
			AmountCredited => "tx_amount_credited",
			AmountDebited => "tx_amount_debited",
			Fee => "tx_fee",
			NetDifference => "tx-net-difference",
			PaymentProof => "tx_payment_proof",
			Kernel => "tx_kernel",
			TxData => "tx_data",
			FuzzyScore => unreachable!("fuzzy score not used as an actual column"),
		};

		s.to_string()
	}
}

impl From<&str> for ColumnKey {
	fn from(s: &str) -> Self {
		match s {
			"tx_id" => ColumnKey::Id,
			"tx_type" => ColumnKey::Type,
			"tx_shared_id" => ColumnKey::SharedTransactionId,
			"tx_creation_time" => ColumnKey::CreationTime,
			"tx-status" => ColumnKey::Status,
			"tx_ttl_cutoff" => ColumnKey::TTLCutoff,
			"tx_height" => ColumnKey::Height,
			"tx_is_confirmed" => ColumnKey::IsConfirmed,
			"tx-confirmation-time" => ColumnKey::ConfirmationTime,
			"tx_num_inputs" => ColumnKey::NumInputs,
			"tx_num_outputs" => ColumnKey::NumOutputs,
			"tx_amount_credited" => ColumnKey::AmountCredited,
			"tx_amount_debited" => ColumnKey::AmountDebited,
			"tx_fee" => ColumnKey::Fee,
			"tx-net-difference" => ColumnKey::NetDifference,
			"tx_payment_proof" => ColumnKey::PaymentProof,
			"tx_kernel" => ColumnKey::Kernel,
			"tx_data" => ColumnKey::TxData,
			_ => panic!("Unknown ColumnKey for {}", s),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
	Asc,
	Desc,
}

impl SortDirection {
	fn toggle(self) -> SortDirection {
		match self {
			SortDirection::Asc => SortDirection::Desc,
			SortDirection::Desc => SortDirection::Asc,
		}
	}
}

#[derive(Debug, Clone)]
pub struct TxLogEntryWrap {
	pub tx: TxLogEntry,
}

impl TxLogEntryWrap {
	pub fn new(tx: TxLogEntry) -> Self {
		Self { tx }
	}
}

#[derive(Debug, Clone)]
pub struct TxList {
	pub txs: Vec<TxLogEntryWrap>,
}

impl Default for TxList {
	fn default() -> Self {
		Self { txs: vec![] }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TxListResultSize {
	_25,
	_50,
	_100,
	_500,
}

impl Default for TxListResultSize {
	fn default() -> Self {
		TxListResultSize::_25
	}
}

impl TxListResultSize {
	pub fn all() -> Vec<TxListResultSize> {
		vec![
			TxListResultSize::_25,
			TxListResultSize::_50,
			TxListResultSize::_100,
			TxListResultSize::_500,
		]
	}

	pub fn as_usize(self) -> usize {
		match self {
			TxListResultSize::_25 => 25,
			TxListResultSize::_50 => 50,
			TxListResultSize::_100 => 100,
			TxListResultSize::_500 => 500,
		}
	}
}

impl std::fmt::Display for TxListResultSize {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut vars = HashMap::new();
		vars.insert("number".to_string(), self.as_usize());
		let fmt = localized_string("tx-list-results");

		write!(f, "{}", strfmt(&fmt, &vars).unwrap())
	}
}

pub struct HeaderState {
	pub state: header::State,
	pub previous_column_key: Option<ColumnKey>,
	pub previous_sort_direction: Option<SortDirection>,
	pub columns: Vec<ColumnState>,
}

impl HeaderState {
	pub fn column_config(&self) -> Vec<(ColumnKey, Length, bool)> {
		self.columns
			.iter()
			.map(|c| (c.key, c.width, c.hidden))
			.collect()
	}
}

impl Default for HeaderState {
	fn default() -> Self {
		Self {
			state: Default::default(),
			previous_column_key: None,
			previous_sort_direction: None,
			columns: vec![
				ColumnState {
					key: ColumnKey::Id,
					// btn_state: Default::default(),
					width: Length::Fixed(20.0),
					hidden: true,
					order: 0,
				},
				ColumnState {
					key: ColumnKey::NetDifference,
					// btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: false,
					order: 1,
				},
				ColumnState {
					key: ColumnKey::CreationTime,
					// btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: false,
					order: 2,
				},
				ColumnState {
					key: ColumnKey::Status,
					// btn_state: Default::default(),
					width: Length::Fixed(300.0),
					hidden: false,
					order: 3,
				},
				ColumnState {
					key: ColumnKey::ConfirmationTime,
					// btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 4,
				},
				ColumnState {
					key: ColumnKey::Type,
					// btn_state: Default::default(),
					width: Length::Fixed(150.0),
					hidden: true,
					order: 5,
				},
				ColumnState {
					key: ColumnKey::SharedTransactionId,
					// btn_state: Default::default(),
					width: Length::Fixed(150.0),
					hidden: true,
					order: 6,
				},
				ColumnState {
					key: ColumnKey::TTLCutoff,
					// btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 7,
				},
				ColumnState {
					key: ColumnKey::Height,
					// btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 8,
				},
				ColumnState {
					key: ColumnKey::IsConfirmed,
					// btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 9,
				},
				ColumnState {
					key: ColumnKey::NumInputs,
					// btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 10,
				},
				ColumnState {
					key: ColumnKey::NumOutputs,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 11,
				},
				ColumnState {
					key: ColumnKey::AmountCredited,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 12,
				},
				ColumnState {
					key: ColumnKey::AmountDebited,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 13,
				},
				ColumnState {
					key: ColumnKey::Fee,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 14,
				},
				ColumnState {
					key: ColumnKey::PaymentProof,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 15,
				},
				ColumnState {
					key: ColumnKey::Kernel,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 16,
				},
				ColumnState {
					key: ColumnKey::TxData,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: true,
					order: 17,
				},
			],
		}
	}
}

pub struct ColumnState {
	key: ColumnKey,
	// btn_state: button::State,
	width: Length,
	hidden: bool,
	order: usize,
}

pub struct ColumnSettings {
	// pub scrollable_state: scrollable::State,
	pub columns: Vec<ColumnSettingState>,
}

impl Default for ColumnSettings {
	fn default() -> Self {
		ColumnSettings {
			// scrollable_state: Default::default(),
			columns: vec![
				ColumnSettingState {
					key: ColumnKey::Id,
					order: 0,
				},
				ColumnSettingState {
					key: ColumnKey::NetDifference,
					order: 1,
				},
				ColumnSettingState {
					key: ColumnKey::CreationTime,
					order: 2,
				},
				ColumnSettingState {
					key: ColumnKey::Status,
					order: 3,
				},
				ColumnSettingState {
					key: ColumnKey::ConfirmationTime,
					order: 4,
				},
				ColumnSettingState {
					key: ColumnKey::Type,
					order: 5,
				},
				ColumnSettingState {
					key: ColumnKey::SharedTransactionId,
					order: 6,
				},
				ColumnSettingState {
					key: ColumnKey::TTLCutoff,
					order: 7,
				},
				ColumnSettingState {
					key: ColumnKey::Height,
					order: 8,
				},
				ColumnSettingState {
					key: ColumnKey::IsConfirmed,
					order: 9,
				},
				ColumnSettingState {
					key: ColumnKey::NumInputs,
					order: 10,
				},
				ColumnSettingState {
					key: ColumnKey::NumOutputs,
					order: 11,
				},
				ColumnSettingState {
					key: ColumnKey::AmountCredited,
					order: 12,
				},
				ColumnSettingState {
					key: ColumnKey::AmountDebited,
					order: 13,
				},
				ColumnSettingState {
					key: ColumnKey::Fee,
					order: 14,
				},
				ColumnSettingState {
					key: ColumnKey::PaymentProof,
					order: 15,
				},
				ColumnSettingState {
					key: ColumnKey::Kernel,
					order: 16,
				},
				ColumnSettingState {
					key: ColumnKey::TxData,
					order: 17,
				},
			],
		}
	}
}

pub struct ColumnSettingState {
	pub key: ColumnKey,
	pub order: usize,
	// pub up_btn_state: button::State,
	// pub down_btn_state: button::State,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum TxListColumnKey {
	Id,
	Type,
	SharedTransactionId,
	CreationTime,
	TTLCutoff,
	Height,
	IsConfirmed,
	ConfirmationTime,
	NumInputs,
	NumOutputs,
	AmountCredited,
	AmountDebited,
	Fee,
	NetDifference,
	PaymentProof,
	Kernel,
	TxData,
}

impl TxListColumnKey {
	fn title(self) -> String {
		use TxListColumnKey::*;

		match self {
			Id => localized_string("tx_id"),
			Type => localized_string("tx_type"),
			SharedTransactionId => localized_string("tx_shared_id"),
			CreationTime => localized_string("tx_creation_time"),
			TTLCutoff => localized_string("tx_ttl_cutoff"),
			Height => localized_string("tx_height"),
			IsConfirmed => localized_string("tx_is_confirmed"),
			ConfirmationTime => localized_string("tx-confirmation-time"),
			NumInputs => localized_string("tx_num_inputs"),
			NumOutputs => localized_string("tx_num_outputs"),
			AmountCredited => localized_string("tx_amount_credited"),
			AmountDebited => localized_string("tx_amount_debited"),
			Fee => localized_string("tx_fee"),
			NetDifference => localized_string("tx-net-difference"),
			PaymentProof => localized_string("tx_payment_proof"),
			Kernel => localized_string("tx_kernel"),
			TxData => localized_string("tx_data"),
		}
	}

	fn as_string(self) -> String {
		use TxListColumnKey::*;

		let s = match self {
			Id => "tx_id",
			Type => "tx_type",
			SharedTransactionId => "tx_shared_id",
			CreationTime => "tx_creation_time",
			TTLCutoff => "tx_ttl_cutoff",
			Height => "tx_height",
			IsConfirmed => "tx_is_confirmed",
			ConfirmationTime => "tx-confirmation-time",
			NumInputs => "tx_num_inputs",
			NumOutputs => "tx_num_outputs",
			AmountCredited => "tx_amount_credited",
			AmountDebited => "tx_amount_debited",
			Fee => "tx_fee",
			NetDifference => "tx-net-difference",
			PaymentProof => "tx_payment_proof",
			Kernel => "tx_kernel",
			TxData => "tx_data",
		};

		s.to_string()
	}
}

impl From<&str> for TxListColumnKey {
	fn from(s: &str) -> Self {
		match s {
			"tx_id" => TxListColumnKey::Id,
			"tx_type" => TxListColumnKey::Type,
			"tx_shared_id" => TxListColumnKey::SharedTransactionId,
			"tx_creation_time" => TxListColumnKey::CreationTime,
			"tx_ttl_cutoff" => TxListColumnKey::TTLCutoff,
			"tx_height" => TxListColumnKey::Height,
			"tx_is_confirmed" => TxListColumnKey::IsConfirmed,
			"tx-confirmation-time" => TxListColumnKey::ConfirmationTime,
			"tx_num_inputs" => TxListColumnKey::NumInputs,
			"tx_num_outputs" => TxListColumnKey::NumOutputs,
			"tx_amount_credited" => TxListColumnKey::AmountCredited,
			"tx_amount_debited" => TxListColumnKey::AmountDebited,
			"tx_fee" => TxListColumnKey::Fee,
			"tx-net-difference" => TxListColumnKey::NetDifference,
			"tx_payment_proof" => TxListColumnKey::PaymentProof,
			"tx_kernel" => TxListColumnKey::Kernel,
			"tx_data" => TxListColumnKey::TxData,
			_ => panic!("Unknown CatalogTxListColumnKey for {}", s),
		}
	}
}
pub struct TxListColumnState {
	key: ColumnKey,
	width: Length,
	hidden: bool,
	order: usize,
}

pub struct TxListHeaderState {
	state: header::State,
	previous_column_key: Option<TxListColumnKey>,
	previous_sort_direction: Option<SortDirection>,
	columns: Vec<TxListColumnState>,
}

impl TxListHeaderState {
	fn column_config(&self) -> Vec<(ColumnKey, Length, bool)> {
		self.columns
			.iter()
			.map(|c| (c.key, c.width, c.hidden))
			.collect()
	}
}

impl Default for TxListHeaderState {
	fn default() -> Self {
		Self {
			state: Default::default(),
			previous_column_key: None,
			previous_sort_direction: None,
			columns: vec![
				TxListColumnState {
					key: ColumnKey::Id,
					//  btn_state: Default::default(),
					width: Length::Fixed(20.0),
					hidden: false,
					order: 0,
				},
				TxListColumnState {
					key: ColumnKey::NetDifference,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 1,
				},
				TxListColumnState {
					key: ColumnKey::CreationTime,
					//  btn_state: Default::default(),
					width: Length::Fixed(105.0),
					hidden: true,
					order: 2,
				},
				TxListColumnState {
					key: ColumnKey::Status,
					//  btn_state: Default::default(),
					width: Length::Fixed(105.0),
					hidden: false,
					order: 3,
				},
				TxListColumnState {
					key: ColumnKey::ConfirmationTime,
					//  btn_state: Default::default(),
					width: Length::Fixed(105.0),
					hidden: false,
					order: 4,
				},
				TxListColumnState {
					key: ColumnKey::Type,
					//  btn_state: Default::default(),
					width: Length::Fixed(150.0),
					hidden: true,
					order: 5,
				},
				TxListColumnState {
					key: ColumnKey::SharedTransactionId,
					//  btn_state: Default::default(),
					width: Length::Fixed(110.0),
					hidden: false,
					order: 6,
				},
				TxListColumnState {
					key: ColumnKey::TTLCutoff,
					//  btn_state: Default::default(),
					width: Length::Fixed(105.0),
					hidden: true,
					order: 7,
				},
				TxListColumnState {
					key: ColumnKey::Height,
					//  btn_state: Default::default(),
					width: Length::Fixed(105.0),
					hidden: false,
					order: 8,
				},
				TxListColumnState {
					key: ColumnKey::IsConfirmed,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: false,
					order: 9,
				},
				TxListColumnState {
					key: ColumnKey::NumInputs,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 10,
				},
				TxListColumnState {
					key: ColumnKey::NumOutputs,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 11,
				},
				TxListColumnState {
					key: ColumnKey::AmountCredited,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 12,
				},
				TxListColumnState {
					key: ColumnKey::AmountDebited,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 13,
				},
				TxListColumnState {
					key: ColumnKey::Fee,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 14,
				},
				TxListColumnState {
					key: ColumnKey::PaymentProof,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 15,
				},
				TxListColumnState {
					key: ColumnKey::Kernel,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 16,
				},
				TxListColumnState {
					key: ColumnKey::TxData,
					//  btn_state: Default::default(),
					width: Length::Fixed(85.0),
					hidden: true,
					order: 17,
				},
			],
		}
	}
}

pub struct TxListColumnSettings {
	pub columns: Vec<TxListColumnSettingState>,
}

impl Default for TxListColumnSettings {
	fn default() -> Self {
		TxListColumnSettings {
			columns: vec![
				TxListColumnSettingState {
					key: ColumnKey::Id,
					order: 0,
				},
				TxListColumnSettingState {
					key: ColumnKey::NetDifference,
					order: 1,
				},
				TxListColumnSettingState {
					key: ColumnKey::CreationTime,
					order: 2,
				},
				TxListColumnSettingState {
					key: ColumnKey::Status,
					order: 3,
				},
				TxListColumnSettingState {
					key: ColumnKey::ConfirmationTime,
					order: 4,
				},
				TxListColumnSettingState {
					key: ColumnKey::Type,
					order: 5,
				},
				TxListColumnSettingState {
					key: ColumnKey::SharedTransactionId,
					order: 6,
				},
				TxListColumnSettingState {
					key: ColumnKey::TTLCutoff,
					order: 7,
				},
				TxListColumnSettingState {
					key: ColumnKey::Height,
					order: 8,
				},
				TxListColumnSettingState {
					key: ColumnKey::IsConfirmed,
					order: 9,
				},
				TxListColumnSettingState {
					key: ColumnKey::NumInputs,
					order: 10,
				},
				TxListColumnSettingState {
					key: ColumnKey::NumOutputs,
					order: 11,
				},
				TxListColumnSettingState {
					key: ColumnKey::AmountCredited,
					order: 12,
				},
				TxListColumnSettingState {
					key: ColumnKey::AmountDebited,
					order: 13,
				},
				TxListColumnSettingState {
					key: ColumnKey::Fee,
					order: 14,
				},
				TxListColumnSettingState {
					key: ColumnKey::PaymentProof,
					order: 15,
				},
				TxListColumnSettingState {
					key: ColumnKey::Kernel,
					order: 16,
				},
				TxListColumnSettingState {
					key: ColumnKey::TxData,
					order: 17,
				},
			],
		}
	}
}

pub struct TxListColumnSettingState {
	pub key: ColumnKey,
	pub order: usize,
}

pub struct CatalogSearchState {
	pub catalog_rows: Vec<CatalogRow>,
	pub query: Option<String>,
	// pub query_state: text_input::State,
	pub result_size: TxListResultSize,
	pub result_sizes: Vec<TxListResultSize>,
	// pub result_sizes_state: pick_list::State<TxListResultSize>,
}

impl Default for CatalogSearchState {
	fn default() -> Self {
		CatalogSearchState {
			catalog_rows: Default::default(),
			query: None,
			// query_state: Default::default(),
			result_size: Default::default(),
			result_sizes: TxListResultSize::all(),
			// result_sizes_state: Default::default(),
		}
	}
}

pub struct CatalogRow {
	// install_button_state: button::State,
}

fn row_title<T: PartialEq>(
	column_key: T,
	previous_column_key: Option<T>,
	previous_sort_direction: Option<SortDirection>,
	title: &str,
) -> String {
	if Some(column_key) == previous_column_key {
		match previous_sort_direction {
			Some(SortDirection::Asc) => format!("{} ▲", title),
			Some(SortDirection::Desc) => format!("{} ▼", title),
			_ => title.to_string(),
		}
	} else {
		title.to_string()
	}
}

pub fn titles_row_header<'a>(
	tx_list: &TxList,
	header_state: &'a header::State,
	column_state: &'a [ColumnState],
	previous_column_key: Option<ColumnKey>,
	previous_sort_direction: Option<SortDirection>,
) -> Header<'a, Message> {
	// A row containing titles above the addon rows.
	let mut row_titles = vec![];

	for column in column_state.iter().filter(|c| !c.hidden) {
		let column_key = column.key;

		let row_title = row_title(
			column_key,
			previous_column_key,
			previous_sort_direction,
			&column.key.title(),
		);

		let mut row_header = Button::new(
			// &mut column.btn_state,
			Text::new(row_title)
				.size(DEFAULT_FONT_SIZE)
				.width(Length::Fill),
		)
		.width(Length::Fill);

		//if column_key != ColumnKey::Install {
		//TODO
		row_header = row_header.on_press(Interaction::SortCatalogColumn(column_key));
		//}

		if previous_column_key == Some(column_key) {
			row_header = row_header.style(grin_gui_core::theme::ButtonStyle::SelectedColumn);
		}
		/*else if column_key == ColumnKey::Install {
			row_header = row_header.style(style::UnclickableColumnHeaderButton);
		} */
		else {
			row_header = row_header.style(grin_gui_core::theme::ButtonStyle::ColumnHeader);
		}

		let row_header: Element<Interaction> = row_header.into();

		let row_container = Container::new(row_header.map(Message::Interaction))
			.width(column.width)
			.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

		// Only shows row titles if we have any catalog results.
		if !tx_list.txs.is_empty() {
			row_titles.push((column.key.as_string(), row_container));
		}
	}

	Header::new(
		header_state.clone(),
		row_titles,
		// Some(Length::Fixed(DEFAULT_PADDING)),
		// Some(Length::Fixed(DEFAULT_PADDING + 5)),
		None,
		None,
	)
	.spacing(1)
	.height(Length::Fixed(25.0))
	.into()
	/* .on_resize(3, |event| {
		//TODO
		//Message::Interaction(Interaction::ResizeColumn(Mode::Catalog, event))
	})*/
}

//TODO: Move somewhere else
pub fn create_tx_display_status(log_entry: &TxLogEntry) -> String {
	if log_entry.confirmed {
		localized_string("tx-confirmed")
	} else {
		localized_string("tx-unconfirmed")
	}
}

#[allow(clippy::too_many_arguments)]
pub fn data_row_container<'a, 'b>(
	tx_log_entry_wrap: &'a TxLogEntryWrap,
	is_tx_expanded: bool,
	expand_type: &'a ExpandType,
	config: &Config,
	column_config: &'b [(ColumnKey, Length, bool)],
	is_odd: Option<bool>,
	pending_confirmation: &Option<Confirm>,
	node_synched: bool,
) -> Container<'a, Message> {
	let default_height = Length::Fixed(26.0);
	let mut default_row_height = 26;

	let mut row_containers = vec![];

	let id = tx_log_entry_wrap.tx.id.to_string();
	let mut tx_type = format!(
		"{}",
		tx_log_entry_wrap.tx.tx_type.to_string().replace("\n", "")
	);
	let shared_tx_id = match tx_log_entry_wrap.tx.tx_slate_id {
		Some(t) => t.to_string(),
		None => "None".to_string(),
	};
	let creation_time = tx_log_entry_wrap.tx.creation_ts.to_string();
	let ttl_cutoff = tx_log_entry_wrap.tx.ttl_cutoff_height;
	let height = tx_log_entry_wrap.tx.kernel_lookup_min_height;

	let tx_cloned = tx_log_entry_wrap.clone();
	let tx_cloned_for_row = tx_log_entry_wrap.clone();

	let creation_time = tx_log_entry_wrap.tx.creation_ts.to_string();
	let confirmation_time = tx_log_entry_wrap.tx.creation_ts.to_string();
	let net_diff = if tx_log_entry_wrap.tx.amount_credited >= tx_log_entry_wrap.tx.amount_debited {
		amount_to_hr_string(
			tx_log_entry_wrap.tx.amount_credited - tx_log_entry_wrap.tx.amount_debited,
			true,
		)
	} else {
		format!(
			"-{}",
			amount_to_hr_string(
				tx_log_entry_wrap.tx.amount_debited - tx_log_entry_wrap.tx.amount_credited,
				true
			)
		)
	};
	//TODO this will show the latest status
	// Unconfirmed - Created time
	// Confirmed
	let status = create_tx_display_status(&tx_log_entry_wrap.tx);

	/*let version = tx
		.version()
		.map(str::to_string)
		.unwrap_or_else(|| "-".to_string());
	let release_package = addon_cloned.relevant_release_package(global_release_channel);*/

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Title && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let title = Text::new(addon.title()).size(DEFAULT_FONT_SIZE);

		let mut title_row = Row::new().push(title).spacing(5).align_items(Align::Center);

		if addon.release_channel != ReleaseChannel::Default {
			let release_channel =
				Container::new(Text::new(addon.release_channel.to_string()).size(10))
					.style(style::ChannelBadge)
					.padding(3);

			title_row = title_row.push(release_channel);
		}

		let mut title_container = Container::new(title_row)
			.padding(5)
			.height(default_height)
			.width(*width)
			.center_y();
		if is_addon_expanded && matches!(expand_type, ExpandType::Details(_)) {
			title_container =
				title_container.style(style::SelectedBrightForegroundContainer);
		} else {
			title_container =
				title_container.style(grin_gui_core::theme::container::Container::HoverableBrightForeground);
		}

		row_containers.push((idx, title_container));
	}*/

	if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Id && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_id = Text::new(id.clone()).size(DEFAULT_FONT_SIZE);

		let id_container = Container::new(display_id)
			.padding(5)
			.height(default_height)
			.width(*width)
			.center_y()
			.style(grin_gui_core::theme::ContainerStyle::HoverableForeground);

		row_containers.push((idx, id_container));
	}

	if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::CreationTime && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_creation_time = Text::new(creation_time).size(DEFAULT_FONT_SIZE);

		let display_creation_time_container = Container::new(display_creation_time)
			.padding(5)
			.height(default_height)
			.width(*width)
			.center_y()
			.style(grin_gui_core::theme::ContainerStyle::HoverableForeground);

		row_containers.push((idx, display_creation_time_container));
	}

	if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::ConfirmationTime && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_confirmation_time = Text::new(confirmation_time).size(DEFAULT_FONT_SIZE);

		let display_confirmation_time_container = Container::new(display_confirmation_time)
			.padding(5)
			.height(default_height)
			.width(*width)
			.center_y()
			.style(grin_gui_core::theme::ContainerStyle::HoverableForeground);

		row_containers.push((idx, display_confirmation_time_container));
	}

	if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::NetDifference && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_net_difference = Text::new(net_diff).size(DEFAULT_FONT_SIZE);

		let display_net_difference_container = Container::new(display_net_difference)
			.padding(5)
			.height(default_height)
			.width(*width)
			.center_y()
			.style(grin_gui_core::theme::ContainerStyle::HoverableForeground);

		row_containers.push((idx, display_net_difference_container));
	}

	if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Status && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_status = Text::new(status).size(DEFAULT_FONT_SIZE);

		let display_status_container = Container::new(display_status)
			.padding(5)
			.height(default_height)
			.width(*width)
			.center_y()
			.style(grin_gui_core::theme::ContainerStyle::HoverableForeground);

		row_containers.push((idx, display_status_container));
	}

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Type && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_type = if let Some(package) = &release_package {
			package.version.clone()
		} else {
			String::from("-")
		};
		let remote_version = Text::new(remote_version).size(DEFAULT_FONT_SIZE);

		let mut remote_version_button =
			Button::new(&mut addon.remote_version_btn_state, remote_version)
				.style(grin_gui_core::theme::button::Button::NormalText);

		if changelog_url.is_some() {
			remote_version_button =
				remote_version_button.on_press(Interaction::Expand(ExpandType::Changelog {
					addon: addon_cloned.clone(),
					changelog: None,
				}));
		}

		let remote_version_button: Element<Interaction> = remote_version_button.into();

		let remote_version_container =
			Container::new(remote_version_button.map(Message::Interaction))
				.height(default_height)
				.width(*width)
				.center_y()
				.style(grin_gui_core::theme::container::Container::HoverableForeground);

		row_containers.push((idx, remote_version_container));
	}*/

	if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Type && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let display_tx_type = Text::new(tx_type.clone()).size(SMALLER_FONT_SIZE);
		let display_tx_type_container = Container::new(display_tx_type)
			.height(default_height)
			.width(*width)
			.center_y()
			.padding(5)
			.style(grin_gui_core::theme::ContainerStyle::HoverableForeground);

		row_containers.push((idx, display_tx_type_container));
	}

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Author && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let author = Text::new(author.as_deref().unwrap_or("-")).size(DEFAULT_FONT_SIZE);
		let author_container = Container::new(author)
			.height(default_height)
			.width(*width)
			.center_y()
			.padding(5)
			.style(grin_gui_core::theme::container::Container::HoverableForeground);

		row_containers.push((idx, author_container));
	}*/

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::GameVersion && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let game_version =
			Text::new(game_version.as_deref().unwrap_or("-")).size(DEFAULT_FONT_SIZE);
		let game_version_container = Container::new(game_version)
			.height(default_height)
			.width(*width)
			.center_y()
			.padding(5)
			.style(grin_gui_core::theme::container::Container::HoverableForeground);

		row_containers.push((idx, game_version_container));
	}*/

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::DateReleased && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let release_date_text: String = if let Some(package) = &release_package {
			let f = localized_timeago_formatter();
			let now = Local::now();

			if let Some(time) = package.date_time.as_ref() {
				f.convert_chrono(*time, now)
			} else {
				"".to_string()
			}
		} else {
			"-".to_string()
		};
		let release_date_text = Text::new(release_date_text).size(DEFAULT_FONT_SIZE);
		let game_version_container = Container::new(release_date_text)
			.height(default_height)
			.width(*width)
			.center_y()
			.padding(5)
			.style(grin_gui_core::theme::container::Container::HoverableForeground);

		row_containers.push((idx, game_version_container));
	}*/

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Source && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let source_text =
			repository_kind.map_or_else(|| localized_string("unknown"), |a| a.to_string());
		let source = Text::new(source_text).size(DEFAULT_FONT_SIZE);
		let source_container = Container::new(source)
			.height(default_height)
			.width(*width)
			.center_y()
			.padding(5)
			.style(grin_gui_core::theme::container::Container::HoverableForeground);

		row_containers.push((idx, source_container));
	}*/

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Summary && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let text = addon_cloned.notes().unwrap_or("-");
		let summary = Text::new(text).size(DEFAULT_FONT_SIZE);
		let container = Container::new(summary)
			.height(default_height)
			.width(*width)
			.center_y()
			.padding(5)
			.style(grin_gui_core::theme::container::Container::HoverableForeground);

		row_containers.push((idx, container));
	}*/

	/*if let Some((idx, width)) = column_config
		.iter()
		.enumerate()
		.filter_map(|(idx, (key, width, hidden))| {
			if *key == ColumnKey::Status && !hidden {
				Some((idx, width))
			} else {
				None
			}
		})
		.next()
	{
		let update_button_container = match &addon.state {
			AddonState::Idle => Container::new(Text::new("".to_string()).size(DEFAULT_FONT_SIZE))
				.height(default_height)
				.width(*width)
				.center_y()
				.center_x()
				.style(grin_gui_core::theme::container::Container::HoverableForeground),
			AddonState::Completed => {
				Container::new(Text::new(localized_string("completed")).size(DEFAULT_FONT_SIZE))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.style(grin_gui_core::theme::container::Container::HoverableForeground)
			}
			AddonState::Error(message) => {
				Container::new(Text::new(message).size(DEFAULT_FONT_SIZE))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.style(grin_gui_core::theme::container::Container::HoverableForeground)
			}
			AddonState::Updatable | AddonState::Retry => {
				let id = addon.primary_folder_id.clone();

				let text = match addon.state {
					AddonState::Updatable => localized_string("update"),
					AddonState::Retry => localized_string("retry"),
					_ => "".to_owned(),
				};

				let update_wrapper = Container::new(Text::new(text).size(DEFAULT_FONT_SIZE))
					.width(*width)
					.center_x()
					.align_x(Align::Center);
				let update_button: Element<Interaction> =
					Button::new(&mut addon.update_btn_state, update_wrapper)
						.width(Length::FillPortion(1))
						.style(style::SecondaryButton)
						.on_press(Interaction::Update(id))
						.into();

				Container::new(update_button.map(Message::Interaction))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.style(grin_gui_core::theme::container::Container::HoverableBrightForeground)
			}
			AddonState::Downloading => {
				Container::new(Text::new(localized_string("downloading")).size(DEFAULT_FONT_SIZE))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.padding(5)
					.style(grin_gui_core::theme::container::Container::HoverableForeground)
			}
			AddonState::Unpacking => {
				Container::new(Text::new(localized_string("unpacking")).size(DEFAULT_FONT_SIZE))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.padding(5)
					.style(grin_gui_core::theme::container::Container::HoverableForeground)
			}
			AddonState::Fingerprint => {
				Container::new(Text::new(localized_string("hashing")).size(DEFAULT_FONT_SIZE))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.padding(5)
					.style(grin_gui_core::theme::container::Container::HoverableForeground)
			}
			AddonState::Ignored => {
				Container::new(Text::new(localized_string("ignored")).size(DEFAULT_FONT_SIZE))
					.height(default_height)
					.width(*width)
					.center_y()
					.center_x()
					.padding(5)
					.style(grin_gui_core::theme::container::Container::HoverableForeground)
			}
			AddonState::Unknown => Container::new(Text::new("").size(DEFAULT_FONT_SIZE))
				.height(default_height)
				.width(*width)
				.center_y()
				.center_x()
				.padding(5)
				.style(grin_gui_core::theme::container::Container::HoverableForeground),
		};

		row_containers.push((idx, update_button_container));
	}*/

	let left_spacer = Space::new(Length::Fixed(DEFAULT_PADDING), Length::Fixed(0.0));
	let right_spacer = Space::new(Length::Fixed(DEFAULT_PADDING + 5.0), Length::Fixed(0.0));

	//let mut row = Row::new().push(left_spacer).spacing(1);
	let mut row = Row::new().spacing(1);

	// Sort columns and push them into row
	row_containers.sort_by(|a, b| a.0.cmp(&b.0));
	for (_, elem) in row_containers.into_iter() {
		row = row.push(elem);
	}

	row = row.push(right_spacer);

	let mut tx_column = Column::new().push(row);
	let mut action_button_row = Row::new();

	if is_tx_expanded {
		match expand_type {
			ExpandType::Details(_) => {
				let button_width = Length::Fixed(BUTTON_WIDTH);

				// ID
				let id_title_text =
					Text::new(format!("{}: ", localized_string("tx-id"))).size(DEFAULT_FONT_SIZE);
				let id_title_container = Container::new(id_title_text)
					.style(grin_gui_core::theme::ContainerStyle::HoverableBrightForeground);

				let id_text = Text::new(id).size(DEFAULT_FONT_SIZE);
				let id_text_container = Container::new(id_text)
					.style(grin_gui_core::theme::ContainerStyle::HoverableBrightForeground);

				let id_row = Row::new()
					.push(id_title_container)
					.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
					.push(id_text_container);

				// UUID
				let uuid_title_text = Text::new(format!("{}: ", localized_string("tx-shared-id")))
					.size(DEFAULT_FONT_SIZE);
				let uuid_title_container = Container::new(uuid_title_text)
					.style(grin_gui_core::theme::ContainerStyle::HoverableBrightForeground);

				let uuid_text = Text::new(shared_tx_id).size(DEFAULT_FONT_SIZE);
				let uuid_text_container = Container::new(uuid_text)
					.style(grin_gui_core::theme::ContainerStyle::HoverableBrightForeground);

				let uuid_row = Row::new()
					.push(uuid_title_container)
					.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
					.push(uuid_text_container);

				// Transaction type
				let type_title_text =
					Text::new(format!("{}: ", localized_string("tx-type"))).size(DEFAULT_FONT_SIZE);
				let type_title_container = Container::new(type_title_text)
					.style(grin_gui_core::theme::ContainerStyle::HoverableBrightForeground);

				let type_text = Text::new(tx_type).size(DEFAULT_FONT_SIZE);
				let type_text_container = Container::new(type_text)
					.style(grin_gui_core::theme::ContainerStyle::HoverableBrightForeground);

				let type_row = Row::new()
					.push(type_title_container)
					.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
					.push(type_text_container);

				/*let notes = notes.unwrap_or_else(|| localized_string("no-addon-description"));
				let author = author.unwrap_or_else(|| "-".to_string());*/
				let left_spacer = Space::new(Length::Fixed(DEFAULT_PADDING), Length::Fixed(0.0));
				let space = Space::new(Length::Fixed(0.0), Length::Fixed(DEFAULT_PADDING * 2.0));
				let bottom_space = Space::new(Length::Fixed(0.0), Length::Fixed(4.0));

				let confirmed = tx_cloned.tx.confirmed;

				let tx_details_container = Container::new(
					Text::new(localized_string("tx-details")).size(DEFAULT_FONT_SIZE),
				)
				.width(button_width)
				.align_y(alignment::Vertical::Center)
				.align_x(alignment::Horizontal::Center);

				let tx_details_button: Element<Interaction> = Button::new(tx_details_container)
					.width(Length::Fixed(BUTTON_WIDTH))
					.style(grin_gui_core::theme::ButtonStyle::Primary)
					.on_press(Interaction::WalletOperationHomeViewInteraction(
						super::home::LocalViewInteraction::TxDetails(tx_cloned.clone()),
					))
					.into();

				let tx_details_wrap =
					Container::new(tx_details_button.map(Message::Interaction)).padding(1.0);
				let tx_details_wrap = Container::new(tx_details_wrap)
					.style(grin_gui_core::theme::ContainerStyle::Segmented)
					.padding(1);

				action_button_row = Row::new()
					.push(Space::new(
						Length::Fixed(DEFAULT_PADDING * 3.0),
						Length::Fixed(0.0),
					))
					.push(tx_details_wrap)
					.push(Space::with_width(Length::Fixed(DEFAULT_PADDING)));

				// Invoice proof view/copy paste
				if config.tx_method == TxMethod::Contracts {
					let tx_proof_container = Container::new(
						Text::new(localized_string("tx-proof")).size(DEFAULT_FONT_SIZE),
					)
					.width(button_width)
					.align_y(alignment::Vertical::Center)
					.align_x(alignment::Horizontal::Center);

					let tx_proof_button: Element<Interaction> = Button::new(tx_proof_container)
						.width(Length::Fixed(BUTTON_WIDTH))
						.style(grin_gui_core::theme::ButtonStyle::Primary)
						.on_press(Interaction::WalletOperationHomeViewInteraction(
							super::home::LocalViewInteraction::TxProof(tx_cloned),
						))
						.into();

					let tx_proof_wrap =
						Container::new(tx_proof_button.map(Message::Interaction)).padding(1.0);
					let tx_proof_wrap = Container::new(tx_proof_wrap)
						.style(grin_gui_core::theme::ContainerStyle::Segmented)
						.padding(1);
					if tx_cloned_for_row.tx.tx_type != TxLogEntryType::TxSelfSpend {
						action_button_row = action_button_row
							.push(tx_proof_wrap)
							.push(Space::with_width(Length::Fixed(DEFAULT_PADDING)));
					}
				}

				if !confirmed {
					// Re-fetch the slate representing the last saved state
					let tx_reload_slate_container = Container::new(
						Text::new(localized_string("tx-reload-slate")).size(DEFAULT_FONT_SIZE),
					)
					.width(Length::Fixed(BUTTON_WIDTH * 2.0))
					.align_y(alignment::Vertical::Center)
					.align_x(alignment::Horizontal::Center);

					let tx_reload_slate_button: Element<Interaction> =
						Button::new(tx_reload_slate_container)
							.width(Length::Fixed(BUTTON_WIDTH * 2.0))
							.style(grin_gui_core::theme::ButtonStyle::Primary)
							.on_press(Interaction::WalletOperationHomeViewInteraction(
								super::home::LocalViewInteraction::ReloadTxSlate(
									tx_cloned_for_row.tx.tx_slate_id.unwrap().to_string(),
								),
							))
							.into();

					let tx_reload_slate_wrap =
						Container::new(tx_reload_slate_button.map(Message::Interaction)).padding(1);
					let tx_reload_slate_wrap = Container::new(tx_reload_slate_wrap)
						.style(grin_gui_core::theme::ContainerStyle::Segmented)
						.padding(1);

					// Present cancel button
					let tx_button_cancel_container = Container::new(
						Text::new(localized_string("cancel-tx")).size(DEFAULT_FONT_SIZE),
					)
					.width(button_width)
					.align_y(alignment::Vertical::Center)
					.align_x(alignment::Horizontal::Center);

					let mut tx_cancel_button = Button::new(tx_button_cancel_container)
						.width(Length::Fixed(BUTTON_WIDTH))
						.style(grin_gui_core::theme::ButtonStyle::Primary);

					if node_synched {
						tx_cancel_button = tx_cancel_button.on_press(
							Interaction::WalletOperationHomeViewInteraction(
								super::home::LocalViewInteraction::CancelTx(
									tx_log_entry_wrap.tx.id,
									tx_log_entry_wrap.tx.tx_slate_id.unwrap().to_string(),
								),
							),
						);
					}
					let tx_cancel_button: Element<Interaction> = tx_cancel_button.into();

					let tx_cancel_wrap =
						Container::new(tx_cancel_button.map(Message::Interaction)).padding(1);
					let tx_cancel_wrap = Container::new(tx_cancel_wrap)
						.style(grin_gui_core::theme::ContainerStyle::Segmented)
						.padding(1);

					if tx_cloned_for_row.tx.tx_type != TxLogEntryType::TxSelfSpend {
						action_button_row = action_button_row
							.push(tx_reload_slate_wrap)
							.push(Space::with_width(Length::Fixed(DEFAULT_PADDING)))
					}

					action_button_row = action_button_row.push(tx_cancel_wrap)
				}

				/*
				let notes_title_text =
					Text::new(localized_string("summary")).size(DEFAULT_FONT_SIZE);
				let notes_text = Text::new(notes).size(DEFAULT_FONT_SIZE);
				let author_text = Text::new(author).size(DEFAULT_FONT_SIZE);
				let author_title_text =
					Text::new(localized_string("authors")).size(DEFAULT_FONT_SIZE);
				let author_title_container = Container::new(author_title_text)
					.style(grin_gui_core::theme::container::Container::HoverableBrightForeground);
				let notes_title_container = Container::new(notes_title_text)
					.style(grin_gui_core::theme::container::Container::HoverableBrightForeground);

				let release_date_text: String = if let Some(package) = &release_package {
					let f = localized_timeago_formatter();
					let now = Local::now();

					if let Some(time) = package.date_time.as_ref() {
						f.convert_chrono(*time, now)
					} else {
						"".to_string()
					}
				} else {
					localized_string("release-channel-no-release")
				};
				let release_date_text = Text::new(release_date_text).size(DEFAULT_FONT_SIZE);
				let release_date_text_container = Container::new(release_date_text)
					.center_y()
					.padding(5)
					.style(grin_gui_core::theme::container::Container::NormalBackground);

				let release_channel_title =
					Text::new(localized_string("remote-release-channel")).size(DEFAULT_FONT_SIZE);
				let release_channel_title_container = Container::new(release_channel_title)
					.style(grin_gui_core::theme::container::Container::NormalBackground);
				let release_channel_list = PickList::new(
					&mut addon.pick_release_channel_state,
					&ReleaseChannel::ALL[..],
					Some(addon.release_channel),
					Message::ReleaseChannelSelected,
				)
				.text_size(14)
				.width(Length::Fixed(100))
				.style(style::PickList);

				let mut website_button = Button::new(
					&mut addon.website_btn_state,
					Text::new(localized_string("website")).size(DEFAULT_FONT_SIZE),
				)
				.style(grin_gui_core::theme::button::Button::Primary);

				if let Some(link) = website_url {
					website_button = website_button.on_press(Interaction::OpenLink(link));
				}

				let website_button: Element<Interaction> = website_button.into();

				let is_ignored = addon.state == AddonState::Ignored;
				let ignore_button_text = if is_ignored {
					Text::new(localized_string("unignore")).size(DEFAULT_FONT_SIZE)
				} else {
					Text::new(localized_string("ignore")).size(DEFAULT_FONT_SIZE)
				};

				let mut ignore_button =
					Button::new(&mut addon.ignore_btn_state, ignore_button_text)
						.on_press(Interaction::Ignore(addon.primary_folder_id.clone()))
						.style(grin_gui_core::theme::button::Button::Primary);

				if is_ignored {
					ignore_button = ignore_button
						.on_press(Interaction::Unignore(addon.primary_folder_id.clone()));
				} else {
					ignore_button = ignore_button
						.on_press(Interaction::Ignore(addon.primary_folder_id.clone()));
				}

				let ignore_button: Element<Interaction> = ignore_button.into();

				let (title, interaction) = if Some(Confirm::DeleteAddon) == *pending_confirmation {
					(
						localized_string("confirm-deletion"),
						Interaction::ConfirmDeleteAddon(addon.primary_folder_id.clone()),
					)
				} else {
					let mut vars = HashMap::new();
					vars.insert("addon".to_string(), addon_cloned.title());
					let fmt = localized_string("delete-addon");

					(strfmt(&fmt, &vars).unwrap(), Interaction::DeleteAddon())
				};

				let delete_button: Element<Interaction> = Button::new(
					&mut addon.delete_btn_state,
					Text::new(title).size(DEFAULT_FONT_SIZE),
				)
				.on_press(interaction)
				.style(style::DefaultDeleteButton)
				.into();

				let (title, interaction) = if Some(Confirm::DeleteSavedVariables)
					== *pending_confirmation
				{
					(
						localized_string("confirm-deletion"),
						Interaction::ConfirmDeleteSavedVariables(addon.primary_folder_id.clone()),
					)
				} else {
					(
						localized_string("delete-addon-saved-variables"),
						Interaction::DeleteSavedVariables(),
					)
				};
				let delete_savedvariables_button: Element<Interaction> = Button::new(
					&mut addon.delete_saved_variables_btn_state,
					Text::new(title).size(DEFAULT_FONT_SIZE),
				)
				.on_press(interaction)
				.style(style::DefaultDeleteButton)
				.into();

				let mut changelog_button = Button::new(
					&mut addon.changelog_btn_state,
					Text::new(localized_string("changelog")).size(DEFAULT_FONT_SIZE),
				)
				.style(grin_gui_core::theme::button::Button::Primary);

				if changelog_url.is_some() {
					changelog_button =
						changelog_button.on_press(Interaction::Expand(ExpandType::Changelog {
							addon: addon_cloned,
							changelog: None,
						}));
				}

				let changelog_button: Element<Interaction> = changelog_button.into();*/

				/*let test_row = Row::new()
				.push(release_channel_list)
				.push(release_date_text_container);*/

				/*let button_row = Row::new()
				.push(Space::new(Length::Fill, Length::Fixed(0.0)))
				.push(website_button.map(Message::Interaction))
				.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
				.push(changelog_button.map(Message::Interaction))
				.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
				.push(ignore_button.map(Message::Interaction))
				.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
				.push(delete_savedvariables_button.map(Message::Interaction))
				.push(Space::new(Length::Fixed(5.0), Length::Fixed(0.0)))
				.push(delete_button.map(Message::Interaction))
				.width(Length::Fill);*/
				let column = Column::new()
					.push(id_row)
					.push(Space::new(Length::Fixed(0.0), Length::Fixed(3.0)))
					.push(uuid_row)
					.push(Space::new(Length::Fixed(0.0), Length::Fixed(3.0)))
					.push(type_row);
				//.push(Space::new(Length::Fixed(0.0), Length::Fixed(3)))
				/* .push(notes_title_container)
				.push(Space::new(Length::Fixed(0.0), Length::Fixed(3)))
				.push(notes_text)
				.push(Space::new(Length::Fixed(0.0), Length::Fixed(15)))
				.push(release_channel_title_container)
				.push(Space::new(Length::Fixed(0.0), Length::Fixed(3)))
				.push(test_row)
				.push(space)
				.push(button_row)*/
				//.push(bottom_space);
				let details_container = Container::new(column)
					.width(Length::Fill)
					.padding(20)
					.style(grin_gui_core::theme::ContainerStyle::NormalBackground);

				let row = Row::new()
					.push(left_spacer)
					.push(details_container)
					.push(Space::new(
						Length::Fixed(DEFAULT_PADDING + 5.0),
						Length::Fixed(0.0),
					))
					.spacing(1);
				tx_column = tx_column
					.push(Space::new(Length::FillPortion(1), Length::Fixed(1.0)))
					.push(row);
			}
			ExpandType::None => {}
		}
	}

	let mut table_row = TableRow::new(tx_column)
		.width(Length::Fill)
		.inner_row_height(default_row_height)
		.on_press(move |_| {
			Message::Interaction(Interaction::WalletOperationTxListInteraction(
				LocalViewInteraction::Expand(ExpandType::Details(tx_cloned_for_row.clone())),
			))
		});

	if is_odd == Some(true) {
		table_row =
			table_row.style(grin_gui_core::style::table_row::TableRowStyle::TableRowAlternate)
	} else {
		table_row = table_row.style(grin_gui_core::style::table_row::TableRowStyle::Default)
	}

	// Due to what feels like an iced-rs bug, don't put buttons within the actual row as they appear
	// to clear their state between the press down and up event if included within the row itself
	// Some kind of fix to the table row widget might rectify this
	let mut return_column = Column::new().push(table_row).push(action_button_row);

	if is_tx_expanded {
		return_column = return_column.push(Space::new(
			Length::Fixed(0.0),
			Length::Fixed(DEFAULT_PADDING * 2.0),
		));
	}

	let return_container = Container::new(return_column);

	return_container
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
	Expand(ExpandType),
}

pub fn handle_message<'a>(
	grin_gui: &mut GrinGui,
	message: LocalViewInteraction,
) -> Result<Command<Message>> {
	let state = &mut grin_gui
		.wallet_state
		.operation_state
		.home_state
		.tx_list_display_state;
	match message {
		LocalViewInteraction::Expand(expand_type) => match &expand_type {
			ExpandType::Details(tx_wrap) => {
				log::debug!("Interaction::Expand(Tx({:?}))", &tx_wrap.tx.id,);
				let should_close = match &state.expanded_type {
					ExpandType::Details(a) => tx_wrap.tx.id == a.tx.id,
					_ => false,
				};

				if should_close {
					state.expanded_type = ExpandType::None;
				} else {
					state.expanded_type = expand_type.clone();
				}
			}
			ExpandType::None => {
				log::debug!("Interaction::Expand(ExpandType::None)");
			}
		},
	}
	Ok(Command::none())
}
