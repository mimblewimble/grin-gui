// Copyright 2021 The Grin Developers
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Logging wrapper to be used throughout all crates in the workspace
use parking_lot::Mutex;

use backtrace::Backtrace;
use std::{panic, thread};

use log::{LevelFilter, Record};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::append::rolling_file::{
	policy::compound::roll::fixed_window::FixedWindowRoller,
	policy::compound::trigger::size::SizeTrigger, policy::compound::CompoundPolicy,
	RollingFileAppender,
};
use log4rs::append::Append;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::writer::simple::SimpleWriter;
use log4rs::encode::Encode;
use log4rs::filter::{threshold::ThresholdFilter, Filter, Response};
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;

use crate::{LogEntry, LoggingConfig};

pub enum LogArea {
	Gui,
	Node,
	Wallet,
}

pub struct LogAreaConfig {
	area: LogArea,
	config: Option<LoggingConfig>,
}

lazy_static! {
	/// Flag to observe whether logging was explicitly initialised (don't output otherwise)
	static ref WAS_INIT: Mutex<bool> = Mutex::new(false);
	/// Static logging configurations for GUI, Wallet, Node
	static ref LOGGING_CONFIGS: Mutex<Vec<LogAreaConfig>> = Mutex::new(vec![
		LogAreaConfig{
			area: LogArea::Gui,
			config: None,
		},
		LogAreaConfig{
			area: LogArea::Node,
			config: None,
		},
		LogAreaConfig{
			area: LogArea::Wallet,
			config: None,
		}
	]);
	/// Handle to logger to change it at runtime
	static ref LOGGER_HANDLE: Mutex<Option<log4rs::Handle>> = Mutex::new(None);
}

const LOGGING_PATTERN: &str = "{d(%Y%m%d %H:%M:%S%.3f)} {h({l})} {M} - {m}{n}";

/// 32 log files to rotate over by default
const DEFAULT_ROTATE_LOG_FILES: u32 = 32 as u32;

/// This filter is rejecting messages that doesn't start with "grin-gui"
#[derive(Debug)]
struct GrinGuiFilter;

impl Filter for GrinGuiFilter {
	fn filter(&self, record: &Record<'_>) -> Response {
		if let Some(module_path) = record.module_path() {
			if module_path.starts_with("grin_gui")
				&& !module_path.starts_with("grin_gui_core::node")
				&& !module_path.starts_with("grin_gui_core::wallet")
			{
				return Response::Neutral;
			}
		}
		Response::Reject
	}
}

/// This filter is rejecting messages that doesn't start with "grin"
/// in order to save log space for only Grin-related records
#[derive(Debug)]
struct GrinFilter;

impl Filter for GrinFilter {
	fn filter(&self, record: &Record<'_>) -> Response {
		if let Some(module_path) = record.module_path() {
			if (module_path.starts_with("grin")
				&& !module_path.starts_with("grin_gui")
				&& !module_path.starts_with("grin_wallet"))
				|| module_path.starts_with("grin_gui_core::node")
			{
				return Response::Neutral;
			}
		}
		Response::Reject
	}
}

/// This filter is rejecting messages that doesn't start with "grin"
/// in order to save log space for only Grin-related records
#[derive(Debug)]
struct GrinWalletFilter;

impl Filter for GrinWalletFilter {
	fn filter(&self, record: &Record<'_>) -> Response {
		if let Some(module_path) = record.module_path() {
			if module_path.starts_with("grin_wallet")
				|| module_path.starts_with("grin_gui_core::wallet")
			{
				return Response::Neutral;
			}
		}
		Response::Reject
	}
}

#[derive(Debug)]
struct ChannelAppender {
	output: Mutex<SyncSender<LogEntry>>,
	encoder: Box<dyn Encode>,
}

impl Append for ChannelAppender {
	fn append(&self, record: &Record) -> Result<(), Box<dyn Error + Sync + Send>> {
		let mut writer = SimpleWriter(Vec::new());
		self.encoder.encode(&mut writer, record)?;

		let log = String::from_utf8_lossy(writer.0.as_slice()).to_string();

		let _ = self.output.lock().try_send(LogEntry {
			log,
			level: record.level(),
		});

		Ok(())
	}

	fn flush(&self) {}
}

/// Update a logging config, and reinitialize loggers with new config
pub fn update_logging_config(area: LogArea, config: LoggingConfig) {
	{
		let mut configs_ref = LOGGING_CONFIGS.lock();
		match area {
			LogArea::Gui => configs_ref[0].config = Some(config),
			LogArea::Node => configs_ref[1].config = Some(config),
			LogArea::Wallet => configs_ref[2].config = Some(config),
		};
	}
	init_loggers(None)
}

/// Initialize the logger with the given configuration
pub fn init_loggers(_logs_tx: Option<mpsc::SyncSender<LogEntry>>) {
	let configs_ref = LOGGING_CONFIGS.lock();

	// Determine minimum logging level for Root logger
	let mut level_minimum = LevelFilter::Off;

	for la in &(*configs_ref) {
		if let Some(c) = &la.config {
			let level_stdout = c.stdout_log_level.to_level_filter();
			let level_file = c.file_log_level.to_level_filter();
			if level_stdout > level_minimum {
				level_minimum = level_stdout;
			}
			if level_file > level_minimum {
				level_minimum = level_file;
			}
		}
	}

	let mut root = Root::builder();
	let mut appenders = vec![];
	let mut info_string = "".to_owned();

	for la in &(*configs_ref) {
		if let Some(c) = &la.config {
			// Start logger
			let stdout = ConsoleAppender::builder()
				.encoder(Box::new(PatternEncoder::new(&LOGGING_PATTERN)))
				.build();

			let level_stdout = c.stdout_log_level.to_level_filter();
			let level_file = c.file_log_level.to_level_filter();

			if c.log_to_stdout {
				let mut builder =
					Appender::builder().filter(Box::new(ThresholdFilter::new(level_stdout)));
				let name = match la.area {
					LogArea::Gui => {
						builder = builder.filter(Box::new(GrinGuiFilter));
						"gui-stdout"
					}
					LogArea::Node => {
						builder = builder.filter(Box::new(GrinFilter));
						"node-stdout"
					}
					LogArea::Wallet => {
						builder = builder.filter(Box::new(GrinWalletFilter));
						"wallet-stdout"
					}
				};
				appenders.push(builder.build(name, Box::new(stdout)));
				root = root.appender(name);
				info_string = format!("{} {} - {},", info_string, name, level_stdout);
			}

			if c.log_to_file {
				// If maximum log size is specified, use rolling file appender
				// or use basic one otherwise
				let filter = Box::new(ThresholdFilter::new(level_file));
				let file: Box<dyn Append> = {
					if let Some(size) = c.log_max_size {
						let count = c.log_max_files.unwrap_or_else(|| DEFAULT_ROTATE_LOG_FILES);
						let roller = FixedWindowRoller::builder()
							.build(&format!("{}.{{}}.gz", c.log_file_path), count)
							.unwrap();
						let trigger = SizeTrigger::new(size);

						let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

						Box::new(
							RollingFileAppender::builder()
								.append(c.log_file_append)
								.encoder(Box::new(PatternEncoder::new(&LOGGING_PATTERN)))
								.build(c.log_file_path.clone(), Box::new(policy))
								.expect("Failed to create logfile"),
						)
					} else {
						Box::new(
							FileAppender::builder()
								.append(c.log_file_append)
								.encoder(Box::new(PatternEncoder::new(&LOGGING_PATTERN)))
								.build(c.log_file_path.clone())
								.expect("Failed to create logfile"),
						)
					}
				};

				let mut builder = Appender::builder().filter(filter);
				let name = match la.area {
					LogArea::Gui => {
						builder = builder.filter(Box::new(GrinGuiFilter));
						"gui-file"
					}
					LogArea::Node => {
						builder = builder.filter(Box::new(GrinFilter));
						"node-file"
					}
					LogArea::Wallet => {
						builder = builder.filter(Box::new(GrinWalletFilter));
						"wallet-file"
					}
				};

				appenders.push(builder.build(name, file));
				root = root.appender(name);
				info_string = format!("{} {} - {},", info_string, name, level_file);
			}
		}
	}

	let config = Config::builder()
		.appenders(appenders)
		.build(root.build(level_minimum));

	// Init or update config via handle
	match config {
		Ok(c) => {
			// Lock handle
			let mut log_handle = LOGGER_HANDLE.lock();
			if let Some(l) = &*log_handle {
				l.set_config(c);
				info!(
					"log4rs configuration changed - {} min level: {}",
					info_string, level_minimum
				);
			} else {
				match log4rs::init_config(c) {
					Ok(h) => {
						*log_handle = Some(h);
						info!(
							"log4rs configuration init - {} min level: {}",
							info_string, level_minimum
						);
					}
					Err(e) => error!("Unable to create logger: {:?}", e),
				}
			}
		}
		Err(e) => error!("Unable to create logging config: {:?}", e),
	}

	send_panic_to_log();
}

/// hook to send panics to logs as well as stderr
fn send_panic_to_log() {
	panic::set_hook(Box::new(|info| {
		let backtrace = Backtrace::new();

		let thread = thread::current();
		let thread = thread.name().unwrap_or("unnamed");

		let msg = match info.payload().downcast_ref::<&'static str>() {
			Some(s) => *s,
			None => match info.payload().downcast_ref::<String>() {
				Some(s) => &**s,
				None => "Box<Any>",
			},
		};

		match info.location() {
			Some(location) => {
				error!(
					"\nthread '{}' panicked at '{}': {}:{}{:?}\n\n",
					thread,
					msg,
					location.file(),
					location.line(),
					backtrace
				);
			}
			None => error!("thread '{}' panicked at '{}'{:?}", thread, msg, backtrace),
		}
		//also print to stderr
		let configs_ref = LOGGING_CONFIGS.lock();

		if let Some(c) = &configs_ref[0].config {
			eprintln!(
				"Thread '{}' panicked with message:\n\"{}\"\nSee {} for further details.",
				thread, msg, c.log_file_path
			);
		}
	}));
}
