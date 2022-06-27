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

use grin_core::global;

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

    pub fn check_initial_config(&mut self) -> Result<Option<&GlobalWalletConfig>, ConfigError> {
        let current_dir = None;
        let create_path = false;

        // Load relevant config, try and load a wallet config file
        self.config = Some(grin_wallet_config::initial_setup_wallet(
            &self.chain_type,
            current_dir,
            create_path,
        )?);

        Ok(self.config.as_ref())
    }

    pub fn init(&mut self) {
        if let None = self.config {
            self.config = Some(grin_wallet_config::initial_setup_wallet(&self.chain_type, None, false).unwrap());
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
            let _ = lc.set_top_level_directory(&wallet_config.data_file_dir);
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
