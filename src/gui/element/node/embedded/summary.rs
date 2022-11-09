use chrono::prelude::Utc;

use iced_aw::Card;

const NANO_TO_MILLIS: f64 = 1.0 / 1_000_000.0;

use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_SUB_HEADER_FONT_SIZE},
    crate::gui::{style, GrinGui, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::node::{ChainTypes, ServerStats, SyncStatus},
    grin_gui_core::theme::ColorPalette,
    iced::{alignment, Alignment, Column, Command, Container, Length, Row, Space, Text},
};

pub struct StateContainer {}

impl Default for StateContainer {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {}

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

// TODO: Localization
fn format_sync_status(sync_status: &SyncStatus) -> String {
    match sync_status {
        SyncStatus::Initial => "Initializing".to_owned(),
        SyncStatus::NoSync => "Running".to_owned(),
        SyncStatus::AwaitingPeers(_) => "Waiting for peers".to_owned(),
        SyncStatus::HeaderSync {
            sync_head,
            highest_height,
            ..
        } => {
            let percent = if *highest_height == 0 {
                0
            } else {
                sync_head.height * 100 / highest_height
            };
            format!("Sync step 1/7: Downloading headers: {}%", percent)
        }
        SyncStatus::TxHashsetPibd {
            aborted: _,
            errored: _,
            completed_leaves,
            leaves_required,
            completed_to_height: _,
            required_height: _,
        } => {
            let percent = if *completed_leaves == 0 {
                0
            } else {
                completed_leaves * 100 / leaves_required
            };
            format!(
                "Sync step 2/7: Downloading Tx state (PIBD) - {} / {} entries - {}%",
                completed_leaves, leaves_required, percent
            )
        }
        SyncStatus::TxHashsetDownload(stat) => {
            if stat.total_size > 0 {
                let percent = stat.downloaded_size * 100 / stat.total_size;
                let start = stat.prev_update_time.timestamp_nanos();
                let fin = Utc::now().timestamp_nanos();
                let dur_ms = (fin - start) as f64 * NANO_TO_MILLIS;

                format!("Sync step 2/7: Downloading {}(MB) chain state for state sync: {}% at {:.1?}(kB/s)",
							stat.total_size / 1_000_000,
							percent,
							if dur_ms > 1.0f64 { stat.downloaded_size.saturating_sub(stat.prev_downloaded_size) as f64 / dur_ms as f64 } else { 0f64 },
					)
            } else {
                let start = stat.start_time.timestamp_millis();
                let fin = Utc::now().timestamp_millis();
                let dur_secs = (fin - start) / 1000;

                format!("Sync step 2/7: Downloading chain state for state sync. Waiting remote peer to start: {}s",
							dur_secs,
					)
            }
        },
        SyncStatus::TxHashsetSetup {
            headers,
            headers_total,
            kernel_pos,
            kernel_pos_total,
        } => {
            if headers.is_some() && headers_total.is_some() {
                let h = headers.unwrap();
                let ht = headers_total.unwrap();
                let percent = h * 100 / ht;
                format!(
                    "Sync step 3/7: Preparing for validation (kernel history) - {}/{} - {}%",
                    h, ht, percent
                )
            } else if kernel_pos.is_some() && kernel_pos_total.is_some() {
                let k = kernel_pos.unwrap();
                let kt = kernel_pos_total.unwrap();
                let percent = k * 100 / kt;
                format!(
                    "Sync step 3/7: Preparing for validation (kernel position) - {}/{} - {}%",
                    k, kt, percent
                )
            } else {
                format!("Sync step 3/7: Preparing chain state for validation")
            }
        }
        SyncStatus::TxHashsetRangeProofsValidation {
            rproofs,
            rproofs_total,
        } => {
            let r_percent = if *rproofs_total > 0 {
                (rproofs * 100) / rproofs_total
            } else {
                0
            };
            format!(
                "Sync step 4/7: Validating chain state - range proofs: {}%",
                r_percent
            )
        }
        SyncStatus::TxHashsetKernelsValidation {
            kernels,
            kernels_total,
        } => {
            let k_percent = if *kernels_total > 0 {
                (kernels * 100) / kernels_total
            } else {
                0
            };
            format!(
                "Sync step 5/7: Validating chain state - kernels: {}%",
                k_percent
            )
        }
        SyncStatus::TxHashsetSave => {
            "Sync step 6/7: Finalizing chain state for state sync".to_owned()
        }
        SyncStatus::TxHashsetDone => {
            "Sync step 6/7: Finalized chain state for state sync".to_owned()
        }
        SyncStatus::BodySync {
            current_height,
            highest_height,
        } => {
            let percent = if *highest_height == 0 {
                0
            } else {
                current_height * 100 / highest_height
            };
            format!("Sync step 7/7: Downloading blocks: {}%", percent)
        }
        SyncStatus::Shutdown => "Shutting down, closing connections".to_owned(),
    }
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
    stats: &'a Option<ServerStats>,
    chain_type: ChainTypes,
) -> Container<'a, Message> {
    fn stat_row<'a>(
        label_text: &str,
        value_text: &str,
        color_palette: ColorPalette,
    ) -> Column<'a, Message> {
        let line_label = Text::new(label_text).size(DEFAULT_FONT_SIZE);

        let line_label_container =
            Container::new(line_label).style(style::NormalBackgroundContainer(color_palette));

        let line_value = Text::new(value_text).size(DEFAULT_FONT_SIZE);

        let line_value_container =
            Container::new(line_value).style(style::NormalBackgroundContainer(color_palette));

        Column::new()
            .push(line_label_container)
            .push(Space::new(Length::Fill, Length::Units(2)))
            .push(line_value_container)
            .push(Space::new(Length::Fill, Length::Units(10)))
            .align_items(Alignment::Center)
    }
    // Basic Info "Box"
    let stats_info_container = match stats {
        Some(s) => {
            let status_line_value = Text::new(&format_sync_status(&s.sync_status))
                .size(DEFAULT_FONT_SIZE)
                .horizontal_alignment(alignment::Horizontal::Center);

            let status_line_value_container = Container::new(status_line_value)
                .style(style::NormalBackgroundContainer(color_palette));

            let status_line_column = Column::new()
                .push(status_line_value_container)
                .align_items(Alignment::Center);

            let status_line_row = Row::new()
                .push(Space::new(Length::Fill, Length::Units(0)))
                .push(status_line_column)
                .push(Space::new(Length::Fill, Length::Units(0)))
                .align_items(Alignment::Center);

            let status_line_title = match chain_type {
                ChainTypes::Testnet => localized_string("status-line-title-test"),
                _ => localized_string("status-line-title-main"),
            };

            let status_line_card = Card::new(
                Text::new(status_line_title).size(DEFAULT_SUB_HEADER_FONT_SIZE),
                status_line_row,
            )
            .style(style::NormalModalCardContainer(color_palette));

            // Basic status
            let connected_peers_row = stat_row(
                &localized_string("connected-peers-label"),
                &format!("{}", &s.peer_count),
                color_palette,
            );
            let disk_usage_row = stat_row(
                &localized_string("disk-usage-label"),
                &format!("{}", &s.disk_usage_gb),
                color_palette,
            );
            let basic_status_column = Column::new().push(connected_peers_row).push(disk_usage_row);
            let basic_status_card = Card::new(
                Text::new(localized_string("basic-status-title")).size(DEFAULT_SUB_HEADER_FONT_SIZE),
                basic_status_column,
            )
            .style(style::NormalModalCardContainer(color_palette));

            // Tip Status
            let header_tip_hash_row = stat_row(
                &localized_string("header-tip-label"),
                &format!("{}", &s.header_stats.last_block_h),
                color_palette,
            );
            let header_chain_height_row = stat_row(
                &localized_string("header-chain-height-label"),
                &format!("{}", &s.header_stats.height),
                color_palette,
            );
            let header_chain_difficulty_row = stat_row(
                &localized_string("header-chain-difficulty-label"),
                &format!("{}", &s.header_stats.total_difficulty),
                color_palette,
            );
            let header_tip_timestamp_row = stat_row(
                &localized_string("header-tip-timestamp-label"),
                &format!("{}", &s.header_stats.latest_timestamp),
                color_palette,
            );
            let header_status_column = Column::new()
                .push(header_tip_hash_row)
                .push(header_chain_height_row)
                .push(header_chain_difficulty_row)
                .push(header_tip_timestamp_row);

            let header_status_card = Card::new(
                Text::new(localized_string("header-status-title")).size(DEFAULT_SUB_HEADER_FONT_SIZE),
                header_status_column,
            )
            .style(style::NormalModalCardContainer(color_palette));

            // Chain status
            let chain_tip_hash_row = stat_row(
                &localized_string("chain-tip-label"),
                &format!("{}", &s.chain_stats.last_block_h),
                color_palette,
            );
            let chain_height_row = stat_row(
                &localized_string("chain-height-label"),
                &format!("{}", &s.chain_stats.height),
                color_palette,
            );
            let chain_difficulty_row = stat_row(
                &localized_string("chain-difficulty-label"),
                &format!("{}", &s.chain_stats.total_difficulty),
                color_palette,
            );
            let chain_tip_timestamp_row = stat_row(
                &localized_string("chain-tip-timestamp-label"),
                &format!("{}", &s.chain_stats.latest_timestamp),
                color_palette,
            );
            let chain_status_column = Column::new()
                .push(chain_tip_hash_row)
                .push(chain_height_row)
                .push(chain_difficulty_row)
                .push(chain_tip_timestamp_row);

            let chain_status_card = Card::new(
                Text::new(localized_string("chain-status-title")).size(DEFAULT_SUB_HEADER_FONT_SIZE),
                chain_status_column,
            )
            .style(style::NormalModalCardContainer(color_palette));

            // TX Pool
            let tx_status_card = match &s.tx_stats {
                Some(t) => {
                    let transaction_pool_size_row = stat_row(
                        &localized_string("transaction-pool-size-label"),
                        &format!("{}", t.tx_pool_size),
                        color_palette,
                    );
                    let stem_pool_size_row = stat_row(
                        &localized_string("stem-pool-size-label"),
                        &format!("{}", t.stem_pool_size),
                        color_palette,
                    );
                    let tx_status_column = Column::new()
                        .push(transaction_pool_size_row)
                        .push(stem_pool_size_row);

                    Card::new(
                        Text::new(localized_string("transaction-pool-title"))
                            .size(DEFAULT_SUB_HEADER_FONT_SIZE),
                        tx_status_column,
                    )
                }
                None => Card::new(
                    Text::new(localized_string("transaction-pool-title")),
                    Column::new(),
                ),
            }
            .style(style::NormalModalCardContainer(color_palette));

            let display_row_1 = Row::new().push(status_line_card).padding(6).spacing(10);
            let display_row_2 = Row::new()
                .push(header_status_card)
                .push(chain_status_card)
                .padding(6)
                .spacing(10);
            let display_row_3 = Row::new()
                .push(basic_status_card)
                .push(tx_status_card)
                .padding(6)
                .spacing(10);

            let status_column = Column::new()
                .push(display_row_1)
                .push(display_row_2)
                .push(display_row_3);

            Container::new(status_column)
        }
        None => Container::new(Column::new()),
    };

    let stats_info_container = stats_info_container.width(Length::Units(600));

    let colum = Column::new()
        .push(Space::new(Length::Units(0), Length::Fill))
        .push(stats_info_container)
        .push(Space::new(Length::Units(0), Length::Fill))
        .align_items(Alignment::Center);

    Container::new(colum)
        .center_y()
        .center_x()
        .width(Length::Fill)
}
