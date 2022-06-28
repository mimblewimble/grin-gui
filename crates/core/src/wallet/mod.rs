/// Placeholder for all wallet calls
/// Should eventually feature async calls that work via local wallet or remote owner API
use std::path::Path;

use grin_wallet::cmd::wallet_args::inst_wallet;
use grin_wallet_api::Owner;
use grin_wallet_config::{self, ConfigError, GlobalWalletConfig};
use grin_wallet_controller::command::{self, GlobalArgs, InitArgs};
use grin_wallet_impls::{DefaultLCProvider, DefaultWalletImpl, HTTPNodeClient};
use grin_wallet_util::grin_core;
use grin_wallet_util::grin_keychain as keychain;
use std::fs;

use std::path::PathBuf;

use grin_core::global;
use dirs;

/// Wallet configuration file name
pub const WALLET_CONFIG_FILE_NAME: &str = "grin-wallet.toml";

const WALLET_LOG_FILE_NAME: &str = "grin-wallet.log";

const GRIN_HOME: &str = ".grin";
/// Wallet data directory
pub const GRIN_WALLET_DIR: &str = "wallet_data";
/// Wallet top level directory
pub const GRIN_WALLET_TOP_LEVEL_DIR: &str = "grin_wallet";
/// Node API secret
pub const API_SECRET_FILE_NAME: &str = ".foreign_api_secret";
/// Owner API secret
pub const OWNER_API_SECRET_FILE_NAME: &str = ".owner_api_secret";

/// TODO - this differs from the default directory in 5.x,
/// need to reconcile this with existing installs somehow

fn get_grin_wallet_default_path(
	chain_type: &global::ChainTypes,
) -> PathBuf {
	// Check if grin dir exists
	let mut grin_path = match dirs::home_dir() {
		Some(p) => p,
		None => PathBuf::new(),
	};
	grin_path.push(GRIN_HOME);
	grin_path.push(chain_type.shortname());
	grin_path.push(GRIN_WALLET_TOP_LEVEL_DIR);

    grin_path
}

pub struct WalletInterface {
    pub chain_type: global::ChainTypes,
    pub config: Option<GlobalWalletConfig>,
}

impl Default for WalletInterface {
    fn default() -> Self {
        Self {
            chain_type: Default::default(),
            config: Default::default(),
        }
    }
}

impl WalletInterface {
    pub fn set_chain_type(&mut self) {
        self.chain_type = global::ChainTypes::Mainnet;
        global::set_local_chain_type(self.chain_type);
        /*let chain_type = if args.is_present("testnet") {
            global::ChainTypes::Testnet
        } else if args.is_present("usernet") {
            global::ChainTypes::UserTesting
        } else {
            global::ChainTypes::Mainnet
        };*/
    }

    pub fn config_exists(&self, path: &str) -> bool {
        grin_wallet_config::config_file_exists(&path)

    }

    pub fn default_config_exists(&self) -> bool {
        self.config_exists(get_grin_wallet_default_path(&self.chain_type).to_str().unwrap())
    }

    pub fn init(&mut self) {
        let data_path = Some(get_grin_wallet_default_path(&self.chain_type));
        if let None = self.config {
            self.config = Some(grin_wallet_config::initial_setup_wallet(&self.chain_type, data_path, true).unwrap());
        }
        let wallet_config = self.config.clone().unwrap().clone().members.unwrap().wallet;
        let node_client =
            HTTPNodeClient::new(&wallet_config.check_node_api_http_addr, None).unwrap(); // Instantiate wallet (doesn't open the wallet)

        let wallet = inst_wallet::<
            DefaultLCProvider<HTTPNodeClient, keychain::ExtKeychain>,
            HTTPNodeClient,
            keychain::ExtKeychain,
        >(wallet_config.clone(), node_client)
        .unwrap_or_else(|e| {
            println!("{}", e);
            std::process::exit(1);
        });

        {
            let mut wallet_lock = wallet.lock();
            let lc = wallet_lock.lc_provider().unwrap();
            let _ = lc.set_top_level_directory(&get_grin_wallet_default_path(&self.chain_type).to_str().unwrap());
        }

        let global_wallet_args = GlobalArgs {
            account: "default".to_owned(),
            api_secret: None,
            node_api_secret: None,
            show_spent: false,
            password: None,
            tls_conf: None,
        };

        let args = InitArgs {
            list_length: 32,
            password: "password".into(),
            config: wallet_config,
            recovery_phrase: None,
            restore: false,
        };

        let mut owner_api = Owner::new(wallet, None);
        command::init(&mut owner_api, &global_wallet_args, args, false);
    }
}
