use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::{
        config::Config,
        theme::ColorPalette,
        wallet::TxLogEntry,
    },
    grin_gui_widgets::{header, Header, TableRow},
    iced::{button, pick_list, scrollable, text_input, Button, Column, Container, Element, Length, Row, Space, Text},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    strfmt::strfmt,
};

#[derive(Debug, Clone)]
pub enum ExpandType {
    Details(TxLogEntry),
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
            CreationTime => localized_string("tx_creation_time"),
            TTLCutoff => localized_string("tx_ttl_cutoff"),
            Height => localized_string("tx_height"),
            IsConfirmed => localized_string("tx_is_confirmed"),
            ConfirmationTime => localized_string("tx_confirmation_time"),
            NumInputs => localized_string("tx_num_inputs"),
            NumOutputs => localized_string("tx_num_outputs"),
            AmountCredited => localized_string("tx_amount_credited"),
            AmountDebited => localized_string("tx_amount_debited"),
            Fee => localized_string("tx_fee"),
            NetDifference => localized_string("tx_net_difference"),
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
            CreationTime => "tx_creation_time",
            TTLCutoff => "tx_ttl_cutoff",
            Height => "tx_height",
            IsConfirmed => "tx_is_confirmed",
            ConfirmationTime => "tx_confirmation_time",
            NumInputs => "tx_num_inputs",
            NumOutputs => "tx_num_outputs",
            AmountCredited => "tx_amount_credited",
            AmountDebited => "tx_amount_debited",
            Fee => "tx_fee",
            NetDifference => "tx_net_difference",
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
            "tx_ttl_cutoff" => ColumnKey::TTLCutoff,
            "tx_height" => ColumnKey::Height,
            "tx_is_confirmed" => ColumnKey::IsConfirmed,
            "tx_confirmation_time" => ColumnKey::ConfirmationTime,
            "tx_num_inputs" => ColumnKey::NumInputs,
            "tx_num_outputs" => ColumnKey::NumOutputs,
            "tx_amount_credited" => ColumnKey::AmountCredited,
            "tx_amount_debited" => ColumnKey::AmountDebited,
            "tx_fee" => ColumnKey::Fee,
            "tx_net_difference" => ColumnKey::NetDifference,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TxList {
    pub txs: Vec<TxLogEntry>,
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
    fn column_config(&self) -> Vec<(ColumnKey, Length, bool)> {
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
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
                ColumnState {
                    key: ColumnKey::Type,
                    btn_state: Default::default(),
                    width: Length::Units(150),
                    hidden: false,
                    order: 1,
                },
                ColumnState {
                    key: ColumnKey::SharedTransactionId,
                    btn_state: Default::default(),
                    width: Length::Units(150),
                    hidden: false,
                    order: 2,
                },
                ColumnState {
                    key: ColumnKey::CreationTime,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: false,
                    order: 3,
                },
                ColumnState {
                    key: ColumnKey::TTLCutoff,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 4,
                },
                ColumnState {
                    key: ColumnKey::Height,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 5,
                },
                ColumnState {
                    key: ColumnKey::IsConfirmed,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 6,
                },
                ColumnState {
                    key: ColumnKey::ConfirmationTime,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 7,
                },
                ColumnState {
                    key: ColumnKey::NumInputs,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 8,
                },
                ColumnState {
                    key: ColumnKey::NumOutputs,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 9,
                },
                ColumnState {
                    key: ColumnKey::AmountCredited,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 10,
                },
                ColumnState {
                    key: ColumnKey::AmountDebited,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 11,
                },
                ColumnState {
                    key: ColumnKey::Fee,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 12,
                },
                ColumnState {
                    key: ColumnKey::NetDifference,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 13,
                },
                ColumnState {
                    key: ColumnKey::PaymentProof,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 14,
                },
                ColumnState {
                    key: ColumnKey::Kernel,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 15,
                },
                ColumnState {
                    key: ColumnKey::TxData,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: true,
                    order: 16,
                },
            ],
        }
    }
}

pub struct ColumnState {
    key: ColumnKey,
    btn_state: button::State,
    width: Length,
    hidden: bool,
    order: usize,
}

pub struct ColumnSettings {
    pub scrollable_state: scrollable::State,
    pub columns: Vec<ColumnSettingState>,
}

impl Default for ColumnSettings {
    fn default() -> Self {
        ColumnSettings {
            scrollable_state: Default::default(),
            columns: vec![
                ColumnSettingState {
                    key: ColumnKey::Id,
                    order: 0,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Type,
                    order: 1,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::SharedTransactionId,
                    order: 2,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::CreationTime,
                    order: 3,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::TTLCutoff,
                    order: 4,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Height,
                    order: 5,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::IsConfirmed,
                    order: 6,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::ConfirmationTime,
                    order: 7,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::NumInputs,
                    order: 8,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::NumOutputs,
                    order: 9,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::AmountCredited,
                    order: 10,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::AmountDebited,
                    order: 11,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Fee,
                    order: 12,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::NetDifference,
                    order: 13,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::PaymentProof,
                    order: 14,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::Kernel,
                    order: 15,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                ColumnSettingState {
                    key: ColumnKey::TxData,
                    order: 16,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
            ],
        }
    }
}

pub struct ColumnSettingState {
    pub key: ColumnKey,
    pub order: usize,
    pub up_btn_state: button::State,
    pub down_btn_state: button::State,
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
            ConfirmationTime => localized_string("tx_confirmation_time"),
            NumInputs => localized_string("tx_num_inputs"),
            NumOutputs => localized_string("tx_num_outputs"),
            AmountCredited => localized_string("tx_amount_credited"),
            AmountDebited => localized_string("tx_amount_debited"),
            Fee => localized_string("tx_fee"),
            NetDifference => localized_string("tx_net_difference"),
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
            ConfirmationTime => "tx_confirmation_time",
            NumInputs => "tx_num_inputs",
            NumOutputs => "tx_num_outputs",
            AmountCredited => "tx_amount_credited",
            AmountDebited => "tx_amount_debited",
            Fee => "tx_fee",
            NetDifference => "tx_net_difference",
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
            "tx_confirmation_time" => TxListColumnKey::ConfirmationTime,
            "tx_num_inputs" => TxListColumnKey::NumInputs,
            "tx_num_outputs" => TxListColumnKey::NumOutputs,
            "tx_amount_credited" => TxListColumnKey::AmountCredited,
            "tx_amount_debited" => TxListColumnKey::AmountDebited,
            "tx_fee" => TxListColumnKey::Fee,
            "tx_net_difference" => TxListColumnKey::NetDifference,
            "tx_payment_proof" => TxListColumnKey::PaymentProof,
            "tx_kernel" => TxListColumnKey::Kernel,
            "tx_data" => TxListColumnKey::TxData,
            _ => panic!("Unknown CatalogTxListColumnKey for {}", s),
        }
    }
}
pub struct TxListColumnState {
    key: ColumnKey,
    btn_state: button::State,
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
                    btn_state: Default::default(),
                    width: Length::Fill,
                    hidden: false,
                    order: 0,
                },
                TxListColumnState {
                    key: ColumnKey::Type,
                    btn_state: Default::default(),
                    width: Length::Units(150),
                    hidden: false,
                    order: 1,
                },
                TxListColumnState {
                    key: ColumnKey::SharedTransactionId,
                    btn_state: Default::default(),
                    width: Length::Units(110),
                    hidden: false,
                    order: 2,
                },
                TxListColumnState {
                    key: ColumnKey::CreationTime,
                    btn_state: Default::default(),
                    width: Length::Units(105),
                    hidden: true,
                    order: 3,
                },
                TxListColumnState {
                    key: ColumnKey::TTLCutoff,
                    btn_state: Default::default(),
                    width: Length::Units(105),
                    hidden: true,
                    order: 4,
                },
                TxListColumnState {
                    key: ColumnKey::Height,
                    btn_state: Default::default(),
                    width: Length::Units(105),
                    hidden: false,
                    order: 5,
                },
                TxListColumnState {
                    key: ColumnKey::IsConfirmed,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: false,
                    order: 6,
                },
                TxListColumnState {
                    key: ColumnKey::ConfirmationTime,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 7,
                },
                TxListColumnState {
                    key: ColumnKey::NumInputs,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 8,
                },
                TxListColumnState {
                    key: ColumnKey::NumOutputs,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 9,
                },
                TxListColumnState {
                    key: ColumnKey::AmountCredited,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 10,
                },
                TxListColumnState {
                    key: ColumnKey::AmountDebited,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 11,
                },
                TxListColumnState {
                    key: ColumnKey::Fee,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 12,
                },
                TxListColumnState {
                    key: ColumnKey::NetDifference,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 13,
                },
                TxListColumnState {
                    key: ColumnKey::PaymentProof,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 14,
                },
                TxListColumnState {
                    key: ColumnKey::Kernel,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 14,
                },
                 TxListColumnState {
                    key: ColumnKey::TxData,
                    btn_state: Default::default(),
                    width: Length::Units(85),
                    hidden: true,
                    order: 14,
                },
             ],
        }
    }
}

pub struct TxListColumnSettings {
    pub scrollable_state: scrollable::State,
    pub columns: Vec<TxListColumnSettingState>,
}

impl Default for TxListColumnSettings {
    fn default() -> Self {
        TxListColumnSettings {
            scrollable_state: Default::default(),
            columns: vec![
                TxListColumnSettingState {
                    key: ColumnKey::Id,
                    order: 0,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::Type,
                    order: 1,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::SharedTransactionId,
                    order: 2,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::CreationTime,
                    order: 3,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::TTLCutoff,
                    order: 4,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::Height,
                    order: 5,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::IsConfirmed,
                    order: 6,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::ConfirmationTime,
                    order: 7,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::NumInputs,
                    order: 8,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::NumOutputs,
                    order: 9,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::AmountCredited,
                    order: 10,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::AmountDebited,
                    order: 11,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::Fee,
                    order: 12,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::NetDifference,
                    order: 13,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::PaymentProof,
                    order: 14,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::Kernel,
                    order: 15,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
                TxListColumnSettingState {
                    key: ColumnKey::TxData,
                    order: 16,
                    up_btn_state: Default::default(),
                    down_btn_state: Default::default(),
                },
            ],
        }
    }
}

pub struct TxListColumnSettingState {
    pub key: ColumnKey,
    pub order: usize,
    pub up_btn_state: button::State,
    pub down_btn_state: button::State,
}

pub struct CatalogSearchState {
    pub catalog_rows: Vec<CatalogRow>,
    pub scrollable_state: scrollable::State,
    pub query: Option<String>,
    pub query_state: text_input::State,
    pub result_size: TxListResultSize,
    pub result_sizes: Vec<TxListResultSize>,
    pub result_sizes_state: pick_list::State<TxListResultSize>,
}

impl Default for CatalogSearchState {
    fn default() -> Self {
        CatalogSearchState {
            catalog_rows: Default::default(),
            scrollable_state: Default::default(),
            query: None,
            query_state: Default::default(),
            result_size: Default::default(),
            result_sizes: TxListResultSize::all(),
            result_sizes_state: Default::default(),
        }
    }
}

pub struct CatalogRow {
    install_button_state: button::State,
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
    color_palette: ColorPalette,
    tx_list: &TxList,
    header_state: &'a mut header::State,
    column_state: &'a mut [ColumnState],
    previous_column_key: Option<ColumnKey>,
    previous_sort_direction: Option<SortDirection>,
) -> Header<'a, Message> {
    // A row containing titles above the addon rows.
    let mut row_titles = vec![];

    for column in column_state.iter_mut().filter(|c| !c.hidden) {
        let column_key = column.key;

        let row_title = row_title(
            column_key,
            previous_column_key,
            previous_sort_direction,
            &column.key.title(),
        );

        let mut row_header = Button::new(
            &mut column.btn_state,
            Text::new(row_title)
                .size(DEFAULT_FONT_SIZE)
                .width(Length::Fill),
        )
        .width(Length::Fill);

        //if column_key != ColumnKey::Install {
            //TODO
            //row_header = row_header.on_press(Interaction::SortCatalogColumn(column_key));
        //}

        if previous_column_key == Some(column_key) {
            row_header = row_header.style(style::SelectedColumnHeaderButton(color_palette));
        } /*else if column_key == ColumnKey::Install {
            row_header = row_header.style(style::UnclickableColumnHeaderButton(color_palette));
        } */ else {
            row_header = row_header.style(style::ColumnHeaderButton(color_palette));
        }

        let row_header: Element<Interaction> = row_header.into();

        let row_container = Container::new(row_header.map(Message::Interaction))
            .width(column.width)
            .style(style::NormalBackgroundContainer(color_palette));

        // Only shows row titles if we have any catalog results.
        if !tx_list.txs.is_empty() {
            row_titles.push((column.key.as_string(), row_container));
        }
    }

    Header::new(
        header_state,
        row_titles,
        Some(Length::Units(DEFAULT_PADDING)),
        Some(Length::Units(DEFAULT_PADDING + 5)),
    )
    .spacing(1)
    .height(Length::Units(25))
    /* .on_resize(3, |event| {
        //TODO
        //Message::Interaction(Interaction::ResizeColumn(Mode::Catalog, event))
    })*/
}

#[allow(clippy::too_many_arguments)]
pub fn data_row_container<'a, 'b>(
    color_palette: ColorPalette,
    tx: &'a mut TxLogEntry,
    is_tx_expanded: bool,
    expand_type: &'a ExpandType,
    config: &Config,
    column_config: &'b [(ColumnKey, Length, bool)],
    is_odd: Option<bool>,
    pending_confirmation: &Option<Confirm>,
) -> TableRow<'a, Message> {
    let default_height = Length::Units(26);
    let default_row_height = 26;

    let mut row_containers = vec![];

    let id = tx.id.to_string();
    let tx_type = tx.tx_type.to_string();
    let shared_tx_id = tx.tx_slate_id;
    let creation_time = tx.creation_ts.to_string();
    let ttl_cutoff = tx.ttl_cutoff_height;
    let height = tx.kernel_lookup_min_height;

    // Check if current addon is expanded.
    /*let tx_cloned = tx.clone();
    let tx_cloned_for_row = tx.clone();
    let version = tx
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
                    .style(style::ChannelBadge(color_palette))
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
                title_container.style(style::SelectedBrightForegroundContainer(color_palette));
        } else {
            title_container =
                title_container.style(style::HoverableBrightForegroundContainer(color_palette));
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
        let display_id = Text::new(id).size(DEFAULT_FONT_SIZE);

        let id_container = Container::new(display_id)
            .padding(5)
            .height(default_height)
            .width(*width)
            .center_y()
            .style(style::HoverableForegroundContainer(color_palette));

        row_containers.push((idx, id_container));
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
                .style(style::NormalTextButton(color_palette));

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
                .style(style::HoverableForegroundContainer(color_palette));

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
        let display_tx_type = Text::new(tx.tx_type.to_string()).size(DEFAULT_FONT_SIZE);
        let display_tx_type_container = Container::new(display_tx_type)
            .height(default_height)
            .width(*width)
            .center_y()
            .padding(5)
            .style(style::HoverableForegroundContainer(color_palette));

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
            .style(style::HoverableForegroundContainer(color_palette));

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
            .style(style::HoverableForegroundContainer(color_palette));

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
            .style(style::HoverableForegroundContainer(color_palette));

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
            .style(style::HoverableForegroundContainer(color_palette));

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
            .style(style::HoverableForegroundContainer(color_palette));

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
                .style(style::HoverableForegroundContainer(color_palette)),
            AddonState::Completed => {
                Container::new(Text::new(localized_string("completed")).size(DEFAULT_FONT_SIZE))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .style(style::HoverableForegroundContainer(color_palette))
            }
            AddonState::Error(message) => {
                Container::new(Text::new(message).size(DEFAULT_FONT_SIZE))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .style(style::HoverableForegroundContainer(color_palette))
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
                        .style(style::SecondaryButton(color_palette))
                        .on_press(Interaction::Update(id))
                        .into();

                Container::new(update_button.map(Message::Interaction))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .style(style::HoverableBrightForegroundContainer(color_palette))
            }
            AddonState::Downloading => {
                Container::new(Text::new(localized_string("downloading")).size(DEFAULT_FONT_SIZE))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .padding(5)
                    .style(style::HoverableForegroundContainer(color_palette))
            }
            AddonState::Unpacking => {
                Container::new(Text::new(localized_string("unpacking")).size(DEFAULT_FONT_SIZE))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .padding(5)
                    .style(style::HoverableForegroundContainer(color_palette))
            }
            AddonState::Fingerprint => {
                Container::new(Text::new(localized_string("hashing")).size(DEFAULT_FONT_SIZE))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .padding(5)
                    .style(style::HoverableForegroundContainer(color_palette))
            }
            AddonState::Ignored => {
                Container::new(Text::new(localized_string("ignored")).size(DEFAULT_FONT_SIZE))
                    .height(default_height)
                    .width(*width)
                    .center_y()
                    .center_x()
                    .padding(5)
                    .style(style::HoverableForegroundContainer(color_palette))
            }
            AddonState::Unknown => Container::new(Text::new("").size(DEFAULT_FONT_SIZE))
                .height(default_height)
                .width(*width)
                .center_y()
                .center_x()
                .padding(5)
                .style(style::HoverableForegroundContainer(color_palette)),
        };

        row_containers.push((idx, update_button_container));
    }*/

    let left_spacer = Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0));
    let right_spacer = Space::new(Length::Units(DEFAULT_PADDING + 5), Length::Units(0));

    let mut row = Row::new().push(left_spacer).spacing(1);

    // Sort columns and push them into row
    row_containers.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, elem) in row_containers.into_iter() {
        row = row.push(elem);
    }

    row = row.push(right_spacer);

    let mut tx_column = Column::new().push(row);

    /*if is_addon_expanded {
        match expand_type {
            ExpandType::Details(_) => {
                let notes = notes.unwrap_or_else(|| localized_string("no-addon-description"));
                let author = author.unwrap_or_else(|| "-".to_string());
                let left_spacer = Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0));
                let space = Space::new(Length::Units(0), Length::Units(DEFAULT_PADDING * 2));
                let bottom_space = Space::new(Length::Units(0), Length::Units(4));
                let notes_title_text =
                    Text::new(localized_string("summary")).size(DEFAULT_FONT_SIZE);
                let notes_text = Text::new(notes).size(DEFAULT_FONT_SIZE);
                let author_text = Text::new(author).size(DEFAULT_FONT_SIZE);
                let author_title_text =
                    Text::new(localized_string("authors")).size(DEFAULT_FONT_SIZE);
                let author_title_container = Container::new(author_title_text)
                    .style(style::HoverableBrightForegroundContainer(color_palette));
                let notes_title_container = Container::new(notes_title_text)
                    .style(style::HoverableBrightForegroundContainer(color_palette));

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
                    .style(style::FadedBrightForegroundContainer(color_palette));

                let release_channel_title =
                    Text::new(localized_string("remote-release-channel")).size(DEFAULT_FONT_SIZE);
                let release_channel_title_container = Container::new(release_channel_title)
                    .style(style::FadedBrightForegroundContainer(color_palette));
                let release_channel_list = PickList::new(
                    &mut addon.pick_release_channel_state,
                    &ReleaseChannel::ALL[..],
                    Some(addon.release_channel),
                    Message::ReleaseChannelSelected,
                )
                .text_size(14)
                .width(Length::Units(100))
                .style(style::PickList(color_palette));

                let mut website_button = Button::new(
                    &mut addon.website_btn_state,
                    Text::new(localized_string("website")).size(DEFAULT_FONT_SIZE),
                )
                .style(style::DefaultButton(color_palette));

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
                        .style(style::DefaultButton(color_palette));

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
                .style(style::DefaultDeleteButton(color_palette))
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
                .style(style::DefaultDeleteButton(color_palette))
                .into();

                let mut changelog_button = Button::new(
                    &mut addon.changelog_btn_state,
                    Text::new(localized_string("changelog")).size(DEFAULT_FONT_SIZE),
                )
                .style(style::DefaultButton(color_palette));

                if changelog_url.is_some() {
                    changelog_button =
                        changelog_button.on_press(Interaction::Expand(ExpandType::Changelog {
                            addon: addon_cloned,
                            changelog: None,
                        }));
                }

                let changelog_button: Element<Interaction> = changelog_button.into();

                let test_row = Row::new()
                    .push(release_channel_list)
                    .push(release_date_text_container);

                let button_row = Row::new()
                    .push(Space::new(Length::Fill, Length::Units(0)))
                    .push(website_button.map(Message::Interaction))
                    .push(Space::new(Length::Units(5), Length::Units(0)))
                    .push(changelog_button.map(Message::Interaction))
                    .push(Space::new(Length::Units(5), Length::Units(0)))
                    .push(ignore_button.map(Message::Interaction))
                    .push(Space::new(Length::Units(5), Length::Units(0)))
                    .push(delete_savedvariables_button.map(Message::Interaction))
                    .push(Space::new(Length::Units(5), Length::Units(0)))
                    .push(delete_button.map(Message::Interaction))
                    .width(Length::Fill);
                let column = Column::new()
                    .push(author_title_container)
                    .push(Space::new(Length::Units(0), Length::Units(3)))
                    .push(author_text)
                    .push(Space::new(Length::Units(0), Length::Units(15)))
                    .push(notes_title_container)
                    .push(Space::new(Length::Units(0), Length::Units(3)))
                    .push(notes_text)
                    .push(Space::new(Length::Units(0), Length::Units(15)))
                    .push(release_channel_title_container)
                    .push(Space::new(Length::Units(0), Length::Units(3)))
                    .push(test_row)
                    .push(space)
                    .push(button_row)
                    .push(bottom_space);
                let details_container = Container::new(column)
                    .width(Length::Fill)
                    .padding(20)
                    .style(style::FadedNormalForegroundContainer(color_palette));

                let row = Row::new()
                    .push(left_spacer)
                    .push(details_container)
                    .push(Space::new(
                        Length::Units(DEFAULT_PADDING + 5),
                        Length::Units(0),
                    ))
                    .spacing(1);

                addon_column = addon_column
                    .push(Space::new(Length::FillPortion(1), Length::Units(1)))
                    .push(row);
            }
            ExpandType::Changelog { changelog, .. } => {
                let left_spacer = Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0));
                let bottom_space = Space::new(Length::Units(0), Length::Units(4));

                let changelog_title_text =
                    Text::new(localized_string("changelog")).size(DEFAULT_FONT_SIZE);
                let changelog_title_container = Container::new(changelog_title_text)
                    .style(style::BrightForegroundContainer(color_palette));

                let changelog_text = match changelog {
                    Some(changelog) => changelog
                        .text
                        .as_ref()
                        .cloned()
                        .unwrap_or_else(|| localized_string("changelog-press-full-changelog")),
                    _ => localized_string("loading"),
                };

                let mut full_changelog_button = Button::new(
                    &mut addon.changelog_btn_state,
                    Text::new(localized_string("full-changelog")).size(DEFAULT_FONT_SIZE),
                )
                .style(style::DefaultButton(color_palette));

                if let Some(url) = &changelog_url {
                    full_changelog_button =
                        full_changelog_button.on_press(Interaction::OpenLink(url.clone()));
                }

                let full_changelog_button: Element<Interaction> = full_changelog_button.into();

                let mut button_row =
                    Row::new().push(Space::new(Length::FillPortion(1), Length::Units(0)));

                if changelog_url.is_some() {
                    button_row = button_row.push(full_changelog_button.map(Message::Interaction));
                }

                let column = Column::new()
                    .push(changelog_title_container)
                    .push(Space::new(Length::Units(0), Length::Units(12)))
                    .push(Text::new(changelog_text).size(DEFAULT_FONT_SIZE))
                    .push(Space::new(Length::Units(0), Length::Units(8)))
                    .push(button_row)
                    .push(bottom_space);

                let details_container = Container::new(column)
                    .width(Length::Fill)
                    .padding(20)
                    .style(style::FadedNormalForegroundContainer(color_palette));

                let row = Row::new()
                    .push(left_spacer)
                    .push(details_container)
                    .push(Space::new(
                        Length::Units(DEFAULT_PADDING + 5),
                        Length::Units(0),
                    ))
                    .spacing(1);

                addon_column = addon_column
                    .push(Space::new(Length::FillPortion(1), Length::Units(1)))
                    .push(row);
            }
            ExpandType::None => {}
        }
    }*/

    let mut table_row = TableRow::new(tx_column)
        .width(Length::Fill)
        .inner_row_height(default_row_height)
        .on_press(move |_| {
            Message::Interaction(Interaction::Expand(ExpandType::Details(
                tx_cloned_for_row.clone()
            )))
        });

    if is_odd == Some(true) {
        table_row = table_row.style(style::TableRowAlternate(color_palette))
    } else {
        table_row = table_row.style(style::TableRow(color_palette))
    }

    table_row
}
