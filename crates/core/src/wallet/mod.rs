/// Placeholder for all wallet calls
/// Should eventually feature async calls that work via local wallet or remote owner API
use std::path::Path;

use grin_wallet::cmd::wallet_args::inst_wallet;
use grin_wallet_api::Owner;
use grin_wallet_config::{self, ConfigError, GlobalWalletConfig, WalletConfig};
use grin_wallet_controller::command::{GlobalArgs, InitArgs};
use grin_wallet_impls::{DefaultLCProvider, DefaultWalletImpl};
use grin_wallet_libwallet::{NodeClient, WalletInst, WalletLCProvider};
use grin_wallet_util::grin_core;
use grin_wallet_util::grin_keychain as keychain;
use grin_wallet_util::grin_util::Mutex;
use std::{
    fs,
    sync::{Arc, RwLock},
};

use std::path::PathBuf;

use dirs;
use grin_core::global;

// Re-exports
pub use global::ChainTypes;
pub use grin_wallet_impls::HTTPNodeClient;

/// Wallet configuration file name
pub const WALLET_CONFIG_FILE_NAME: &str = "grin-wallet.toml";

const WALLET_LOG_FILE_NAME: &str = "grin-wallet.log";

const GRIN_HOME: &str = ".grin";
/// Wallet data directory
pub const GRIN_WALLET_DIR: &str = "wallet_data";
/// Wallet top level directory
pub const GRIN_WALLET_TOP_LEVEL_DIR: &str = "grin_wallet";
/// Wallet top level directory
pub const GRIN_WALLET_DEFAULT_DIR: &str = "default";
/// Node API secret
pub const API_SECRET_FILE_NAME: &str = ".foreign_api_secret";
/// Owner API secret
pub const OWNER_API_SECRET_FILE_NAME: &str = ".owner_api_secret";

/// TODO - this differs from the default directory in 5.x,
/// need to reconcile this with existing installs somehow

fn get_grin_wallet_default_path(chain_type: &global::ChainTypes) -> PathBuf {
    // Check if grin dir exists
    let mut grin_path = match dirs::home_dir() {
        Some(p) => p,
        None => PathBuf::new(),
    };
    grin_path.push(GRIN_HOME);
    grin_path.push(chain_type.shortname());
    grin_path.push(GRIN_WALLET_TOP_LEVEL_DIR);
    grin_path.push(GRIN_WALLET_DEFAULT_DIR);

    grin_path
}

pub type WalletInterfaceHttpNodeClient = WalletInterface<
    DefaultLCProvider<'static, HTTPNodeClient, keychain::ExtKeychain>,
    HTTPNodeClient,
>;

pub struct WalletInterface<L, C>
where
    L: WalletLCProvider<'static, C, keychain::ExtKeychain> + 'static,
    C: NodeClient + 'static + Clone,
{
    pub chain_type: global::ChainTypes,
    pub config: Arc<RwLock<Option<GlobalWalletConfig>>>,
    // owner api will hold instantiated/opened wallets
    pub owner_api: Option<Owner<L, C, keychain::ExtKeychain>>,
    node_client: C,
}

impl<L, C> WalletInterface<L, C>
where
    L: WalletLCProvider<'static, C, keychain::ExtKeychain>,
    C: NodeClient + 'static + Clone,
{
    pub fn new(node_client: C, chain_type: global::ChainTypes) -> Self {
        WalletInterface {
            chain_type,
            config: Arc::new(RwLock::new(None)),
            owner_api: None,
            node_client,
        }
    }

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
        self.config_exists(
            get_grin_wallet_default_path(&self.chain_type)
                .to_str()
                .unwrap(),
        )
    }

    pub async fn init(
        wallet_interface: Arc<RwLock<WalletInterface<L, C>>>,
        password: String,
    ) -> Result<(String, String), grin_wallet_controller::Error> {
        let mut w = wallet_interface.write().unwrap();

        let data_path = Some(get_grin_wallet_default_path(&w.chain_type));

        let config =
            grin_wallet_config::initial_setup_wallet(&w.chain_type, data_path, true).unwrap();

        let wallet_config = config.clone().members.unwrap().wallet;
        let wallet_inst =
            inst_wallet(wallet_config.clone(), w.node_client.clone()).unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });

        {
            let mut wallet_lock = wallet_inst.lock();
            let lc = wallet_lock.lc_provider().unwrap();
            let _ = lc.set_top_level_directory(
                &get_grin_wallet_default_path(&w.chain_type)
                    .to_str()
                    .unwrap(),
            );
        }

        w.owner_api = Some(Owner::new(wallet_inst.clone(), None));

        let args = InitArgs {
            list_length: 32,
            password: "".into(),
            config: config.clone().members.unwrap().wallet,
            recovery_phrase: None,
            restore: false,
        };

        global::set_local_chain_type(w.chain_type);

        // Assume global chain type has already been initialized.
        let chain_type = global::get_chain_type();

        let (tld, ret_phrase) = match w.owner_api.as_ref() {
            Some(o) => {
                let tld = {
                    let mut w_lock = o.wallet_inst.lock();
                    let p = w_lock.lc_provider()?;
                    p.create_config(&chain_type, WALLET_CONFIG_FILE_NAME, None, None, None)?;
                    p.create_wallet(
                        None,
                        args.recovery_phrase,
                        args.list_length,
                        password.clone().into(),
                        false,
                    )?;
                    p.get_top_level_directory()?
                };
                (tld, o.get_mnemonic(None, password.into())?.to_string())
            }
            None => ("".to_string(), "".to_string()),
        };

        Ok((tld, ret_phrase))
    }

    /*pub async fn get_recovery_phrase(wallet_interface: Arc<RwLock<WalletInterface<L, C>>>, password: String) -> String {
        let mut w = wallet_interface.read().unwrap();
        w.owner_api.get_mnemonic(name, password.into())
    }*/
}
