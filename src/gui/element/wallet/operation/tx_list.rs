use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::{
        theme::ColorPalette,
        wallet::TxLogEntry,
    },
    grin_gui_widgets::{header, Header},
    iced::{button, pick_list, scrollable, text_input, Button, Container, Element, Length, Text},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    strfmt::strfmt,
};


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
    state: header::State,
    previous_column_key: Option<ColumnKey>,
    previous_sort_direction: Option<SortDirection>,
    columns: Vec<ColumnState>,
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
    column_state: &'a mut [TxListColumnState],
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
