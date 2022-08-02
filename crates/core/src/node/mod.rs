use grin_config::{config, GlobalConfig};
use grin_core::global;
use grin_servers as servers;
use grin_util::logger::LogEntry;
use servers::Server;

use futures::channel::oneshot;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use chrono::prelude::Utc;

use crate::logger;

pub use global::ChainTypes;

use iced_futures::futures::channel::mpsc as iced_mpsc;
use iced_futures::futures::SinkExt;
use subscriber::UIMessage;

pub mod subscriber;

// include build information
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn info_strings() -> (String, String) {
    (
        format!(
            "This is Grin version {}{}, built for {} by {}.",
            built_info::PKG_VERSION,
            built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
            built_info::TARGET,
            built_info::RUSTC_VERSION,
        ),
        format!(
            "Built with profile \"{}\", features \"{}\".",
            built_info::PROFILE,
            built_info::FEATURES_STR,
        ),
    )
}

fn log_build_info() {
    let (basic_info, detailed_info) = info_strings();
    info!("{}", basic_info);
    debug!("{}", detailed_info);
}

fn log_feature_flags() {
    info!("Feature: NRD kernel enabled: {}", global::is_nrd_enabled());
}

pub struct Controller {
    logs_rx: mpsc::Receiver<LogEntry>,
    rx_controller: mpsc::Receiver<ControllerMessage>,
    tx_ui: iced_mpsc::Sender<UIMessage>,
}

pub enum ControllerMessage {
    Shutdown,
}

/// This needs to provide the interface in to the server, bridging between the UI and
/// server instance
impl Controller {
    /// Create a new controller
    pub fn new(
        logs_rx: mpsc::Receiver<LogEntry>,
        tx_ui: iced_mpsc::Sender<UIMessage>,
    ) -> Result<Controller, String> {
        let (tx_controller, rx_controller) = mpsc::channel::<ControllerMessage>();
        Ok(Controller { logs_rx, rx_controller, tx_ui })
    }

    /// Run the controller
    pub fn run(&mut self, server: Server) {
        let stat_update_interval = 1;
        let mut next_stat_update = Utc::now().timestamp() + stat_update_interval;
        let delay = Duration::from_millis(50);
        //while self.ui.step() {
        while true {
            if let Some(message) = self.rx_controller.try_iter().next() {
                match message {
                    ControllerMessage::Shutdown => {
                        warn!("Shutdown in progress, please wait");
                        //self.ui.stop();
                        //server.stop();
                        return;
                    }
                }
            }

            if Utc::now().timestamp() > next_stat_update {
                next_stat_update = Utc::now().timestamp() + stat_update_interval;
                if let Ok(stats) = server.get_server_stats() {
                    if let Err(e) = self.tx_ui.try_send(UIMessage::UpdateStatus(stats)) {
                        error!("Unable to send stat message to UI: {}", e);
                    }
                }
            }
            thread::sleep(delay);
        }
        server.stop();
    }
}

pub struct NodeInterface {
    pub chain_type: global::ChainTypes,
    pub config: Option<GlobalConfig>,
    pub ui_sender: Option<iced_mpsc::Sender<UIMessage>>, //pub ui_rx: mpsc::Receiver<UIMessage>,
}

impl NodeInterface {
    pub fn new(chain_type: global::ChainTypes) -> Self {
        NodeInterface {
            chain_type,
            config: None,
            ui_sender: None,
        }
    }

    pub fn set_ui_sender(&mut self, ui_sender: iced_mpsc::Sender<UIMessage>) {
        self.ui_sender = Some(ui_sender)
    }

    pub fn set_chain_type(&mut self) {
        self.chain_type = global::ChainTypes::Mainnet;
        global::set_local_chain_type(self.chain_type);
    }

    pub fn start_server(&mut self) {
        let node_config = Some(
            config::initial_setup_server(&self.chain_type).unwrap_or_else(|e| {
                panic!("Error loading server configuration: {}", e);
            }),
        );
        let config = node_config.clone().unwrap();
        let mut logging_config = config.members.as_ref().unwrap().logging.clone().unwrap();
        logging_config.tui_running = Some(false);

        let api_chan: &'static mut (oneshot::Sender<()>, oneshot::Receiver<()>) =
            Box::leak(Box::new(oneshot::channel::<()>()));

        let (logs_tx, logs_rx) = {
            let (logs_tx, logs_rx) = mpsc::sync_channel::<LogEntry>(200);
            (Some(logs_tx), Some(logs_rx))
        };

        logger::update_logging_config(logger::LogArea::Node, logging_config);

        if let Some(file_path) = &config.config_file_path {
            info!(
                "Using configuration file at {}",
                file_path.to_str().unwrap()
            );
        } else {
            info!("Node configuration file not found, using default");
        };

        log_build_info();
        global::init_global_chain_type(config.members.as_ref().unwrap().server.chain_type);
        info!("Chain: {:?}", global::get_chain_type());
        match global::get_chain_type() {
            global::ChainTypes::Mainnet => {
                // Set various mainnet specific feature flags.
                global::init_global_nrd_enabled(false);
            }
            _ => {
                // Set various non-mainnet feature flags.
                global::init_global_nrd_enabled(true);
            }
        }
        let afb = config
            .members
            .as_ref()
            .unwrap()
            .server
            .pool_config
            .accept_fee_base;
        global::init_global_accept_fee_base(afb);
        info!("Accept Fee Base: {:?}", global::get_accept_fee_base());
        global::init_global_future_time_limit(config.members.unwrap().server.future_time_limit);
        info!("Future Time Limit: {:?}", global::get_future_time_limit());
        log_feature_flags();

        let mut server_config = node_config
            .unwrap()
            .members
            .as_ref()
            .unwrap()
            .server
            .clone();

        let ui_sender = self.ui_sender.as_ref().unwrap().clone();

        thread::Builder::new()
            .name("node_runner".to_string())
            .spawn(move || {
                servers::Server::start(
                    server_config,
                    logs_rx,
                    |serv: servers::Server, logs_rx: Option<mpsc::Receiver<LogEntry>>| {
                        let mut controller =
                            Controller::new(logs_rx.unwrap(), ui_sender.clone()).unwrap_or_else(|e| {
                                error!("Error loading UI controller: {}", e);
                                panic!("Error loading UI controller: {}", e);
                            });
                        controller.run(serv);
                    },
                    None,
                    api_chan,
                )
                .unwrap();
            })
            .ok();
    }
}
