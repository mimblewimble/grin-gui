use {
    super::{GrinGui, Interaction, Message, Mode},
    crate::{gui::element, log_error, Result},
    grin_gui_core::{fs::PersistentData, node::subscriber::UIMessage, node::ChainTypes::Testnet, node::ChainTypes::Mainnet},
    iced::{clipboard, Command},
    //grin_gui_widgets::header::ResizeEvent,
    std::path::PathBuf,
};

#[cfg(target_os = "windows")]
use crate::tray::{TrayMessage, SHOULD_EXIT, TRAY_SENDER};
#[cfg(target_os = "windows")]
use std::sync::atomic::Ordering;

pub fn handle_message(grin_gui: &mut GrinGui, message: Message) -> Result<Command<Message>> {
    if let Some(index) = grin_gui.config.current_wallet_index {
        // Take opportunity to check if we don't have a wallet config file for some reason
        if !grin_gui.wallet_state.config_missing() {
            match &grin_gui.config.wallets[index].tld {
                Some(t) => {
                    let wallet_interface = grin_gui.wallet_interface.clone();
                    let w = wallet_interface.read().unwrap();
                    if !w.config_exists(t.to_str().unwrap()) {
                        grin_gui.wallet_state.set_config_missing();
                    }
                }
                None => {
                    grin_gui.wallet_state.set_config_missing();
                }
            }
        }

        // Check if password needs entering in wallet mode
        if !grin_gui.wallet_state.config_missing()
            && !grin_gui.wallet_state.operation_state.wallet_not_open()
        {
            let w = grin_gui.wallet_interface.read().unwrap();
            if !w.wallet_is_open() {
                grin_gui.wallet_state.operation_state.set_wallet_not_open()
            }
        }
        // Check if embedded node needs starting
        if grin_gui.config.wallets[index].use_embedded_node {
            let (node_started, has_ui_sender) = {
                let n = grin_gui.node_interface.read().unwrap();
                (n.node_started, n.ui_sender.is_some())
            };

            if !node_started && has_ui_sender {
                let mut node = grin_gui.node_interface.write().unwrap();
                //let is_testnet = grin_gui.config.wallets[index].is_testnet;
                let wallet_chain_type = grin_gui.config.wallets[index].chain_type;

                if !node_started {
                    node.start_server(wallet_chain_type);
                
                } else {
                    let running_chain_type = {
                        let node = grin_gui.node_interface.read().unwrap();
                        node.chain_type
                    }.unwrap();

                    if running_chain_type != wallet_chain_type {
                        node.restart_server(wallet_chain_type);
                    }
                }
            }
        }
    } else {
        if !grin_gui.wallet_state.config_missing() {
            grin_gui.wallet_state.set_config_missing();
        }
    }

    // Clear errors when necessary
    match message {
        Message::Interaction(Interaction::OpenErrorModal) => {}
        Message::Interaction(Interaction::CloseErrorModal) => {}
        Message::Interaction(Interaction::WriteToClipboard(_)) => {}
        Message::Interaction(_) => {
            grin_gui.error.take();
        }
        _ => {}
    }

    match message {
        // Ticks, for stuff that happens frequently, like checking wallet status
        Message::Tick(time) => {
            // Call all views 'registered' for ticks
            return element::wallet::operation::home::handle_tick(grin_gui, time);
        }
        // Update from embedded node server
        Message::SendNodeMessage((_id, msg, sender)) => match sender {
            Some(sender) => {
                let mut n = grin_gui.node_interface.write().unwrap();
                n.set_ui_sender(sender);
                return Ok(Command::none());
            }
            None => {
                match msg {
                    UIMessage::None => {}
                    UIMessage::UpdateStatus(stats) => {
                        grin_gui.node_state.embedded_state.server_stats = Some(stats);
                    }
                }
                return Ok(Command::none());
            }
        },
        // Error modal state
        Message::Interaction(Interaction::OpenErrorModal) => grin_gui.modal_state.show(true),
        Message::Interaction(Interaction::CloseErrorModal) => {
            grin_gui.modal_state.show(false)
        }
        // Clipboard messages
        Message::Interaction(Interaction::WriteToClipboard(contents)) => {
            return Ok(clipboard::write::<Message>(contents));
        }
        // Top level menu
        Message::Interaction(Interaction::MenuViewInteraction(l)) => {
            let _ = element::menu::handle_message(grin_gui, l);
        }
        // Top level settings view
        Message::Interaction(Interaction::SettingsViewInteraction(l)) => {
            element::settings::handle_message(grin_gui, l);
        }
        // Settings -> Wallet Settings
        Message::Interaction(Interaction::WalletSettingsViewInteraction(l)) => {
            element::settings::wallet::handle_message(grin_gui, l);
        }
        // Settings -> Node Settings
        Message::Interaction(Interaction::NodeSettingsViewInteraction(l)) => {
            element::settings::node::handle_message(grin_gui, l);
        }

        // Settings -> General Settings
        Message::Interaction(Interaction::GeneralSettingsViewInteraction(l)) => {
            return element::settings::general::handle_message(grin_gui, l);
        }
        // Setup Top Level
        Message::Interaction(Interaction::WalletSetupViewInteraction(l)) => {
            return element::wallet::setup::handle_message(grin_gui, l);
        }
        // Setup -> Initial View (To appear when no wallet toml file is set)
        Message::Interaction(Interaction::WalletSetupInitViewInteraction(l)) => {
            return element::wallet::setup::init::handle_message(grin_gui, l);
        }
        // Setup -> Wallet Init Settings
        Message::Interaction(Interaction::WalletSetupWalletViewInteraction(l)) => {
            return element::wallet::setup::wallet_setup::handle_message(grin_gui, l);
        }
        // Setup -> Wallet List
        Message::Interaction(Interaction::WalletListWalletViewInteraction(l)) => {
            return element::wallet::setup::wallet_list::handle_message(grin_gui, l);
        }
        // Setup -> Wallet Success Settings
        Message::Interaction(Interaction::WalletSetupWalletSuccessViewInteraction(l)) => {
            return element::wallet::setup::wallet_success::handle_message(grin_gui, l);
        }
        // Wallet -> Operation -> Open Settings
        Message::Interaction(Interaction::WalletOperationOpenViewInteraction(l)) => {
            return element::wallet::operation::open::handle_message(grin_gui, l);
        }
        // Wallet -> Operation -> Home Settings
        Message::Interaction(Interaction::WalletOperationHomeViewInteraction(l)) => {
            return element::wallet::operation::home::handle_message(grin_gui, l);
        }
        // Wallet -> Operation -> Home -> TxListDisplay Settings
        Message::Interaction(Interaction::WalletOperationHomeTxListDisplayInteraction(l)) => {
            return element::wallet::operation::tx_list_display::handle_message(grin_gui, l);
        }
         // Wallet -> Operation -> TxList
        Message::Interaction(Interaction::WalletOperationTxListInteraction(l)) => {
            return element::wallet::operation::tx_list::handle_message(grin_gui, l);
        }
        // Wallet -> Operation -> CreateTx
        Message::Interaction(Interaction::WalletOperationCreateTxViewInteraction(l)) => {
            return element::wallet::operation::create_tx::handle_message(grin_gui, l);
        }
        // Wallet -> Operation -> Home -> Action
        Message::Interaction(Interaction::WalletOperationApplyTxViewInteraction(l)) => {
            return element::wallet::operation::apply_tx::handle_message(grin_gui, l);
        }
        // Wallet -> Operation -> Home -> Action
        Message::Interaction(Interaction::WalletOperationHomeActionMenuViewInteraction(l)) => {
            return element::wallet::operation::action_menu::handle_message(grin_gui, l);
        }
          Message::Interaction(Interaction::ModeSelected(mode)) => {
            log::debug!("Interaction::ModeSelected({:?})", mode);
            // Set Mode
            grin_gui.mode = mode;
        }
        /*Message::MessageInteraction(m) => {
            m.handle_message()
        }*/
        Message::Interaction(Interaction::ModeSelectedSettings(mode)) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            //grin_gui.settings_state.mode = mode;
        }
        Message::Error(error) => {
            let mut e = error.write().unwrap();
            let err = e.take();
            if let Some(ref e) = err {
                log_error(e);
            }
            grin_gui.error = err;
        }
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::Resized { width, height },
        )) => {
            let width = (width as f64 * grin_gui.general_settings_state.scale_state.scale) as u32;
            let height = (height as f64 * grin_gui.general_settings_state.scale_state.scale) as u32;

            // Minimizing Grin GUI on Windows will call this function with 0, 0.
            // We don't want to save that in config, because then it will start with zero size.
            if width > 0 && height > 0 {
                grin_gui.config.window_size = Some((width, height));
                let _ = grin_gui.config.save();
            }
        }

        #[cfg(target_os = "macos")]
        // Application shutdown
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::CloseRequested,
        )) => {
            log::debug!("Message::RuntimeEvent(CloseRequested)");
            grin_gui.show_exit(true);
        }

        #[cfg(target_os = "windows")]
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::CloseRequested,
        )) => {
            log::debug!("Message::RuntimeEvent(CloseRequested)");

            if let Some(sender) = TRAY_SENDER.get() {
                if grin_gui.config.close_to_tray {
                    let _ = sender.try_send(TrayMessage::CloseToTray);
                } else {
                    SHOULD_EXIT.store(true, Ordering::Relaxed);
                }
            }
        }
        Message::RuntimeEvent(iced_native::Event::Keyboard(
            iced_native::keyboard::Event::KeyReleased {
                key_code,
                modifiers,
            },
        )) => {
            // Bail out of keybindings if keybindings is diabled, or we are
            // pressing any modifiers.
            if !grin_gui.config.is_keybindings_enabled
                || modifiers != iced::keyboard::Modifiers::default()
            {
                return Ok(Command::none());
            }

            match key_code {
                iced::keyboard::KeyCode::A => {}
                iced::keyboard::KeyCode::C => {
                    grin_gui.mode = Mode::Catalog;
                }
                iced::keyboard::KeyCode::R => {}
                iced::keyboard::KeyCode::S => {
                    grin_gui.mode = Mode::Settings;
                }
                iced::keyboard::KeyCode::U => {}
                iced::keyboard::KeyCode::W => {}
                iced::keyboard::KeyCode::I => {
                    grin_gui.mode = Mode::Install;
                }
                iced::keyboard::KeyCode::Escape => match grin_gui.mode {
                    _ => (),
                },
                _ => (),
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleCloseToTray(enable)) => {
            log::debug!("Interaction::ToggleCloseToTray({})", enable);

            grin_gui.config.close_to_tray = enable;

            // Remove start closed to tray if we are disabling
            if !enable {
                grin_gui.config.start_closed_to_tray = false;
            }

            let _ = grin_gui.config.save();

            if let Some(sender) = TRAY_SENDER.get() {
                let msg = if enable {
                    TrayMessage::Enable
                } else {
                    TrayMessage::Disable
                };

                let _ = sender.try_send(msg);
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleAutoStart(enable)) => {
            log::debug!("Interaction::ToggleAutoStart({})", enable);

            grin_gui.config.autostart = enable;

            let _ = grin_gui.config.save();

            if let Some(sender) = TRAY_SENDER.get() {
                let _ = sender.try_send(TrayMessage::ToggleAutoStart(enable));
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleStartClosedToTray(enable)) => {
            log::debug!("Interaction::ToggleStartClosedToTray({})", enable);

            grin_gui.config.start_closed_to_tray = enable;

            // Enable tray if this feature is enabled
            if enable && !grin_gui.config.close_to_tray {
                grin_gui.config.close_to_tray = true;

                if let Some(sender) = TRAY_SENDER.get() {
                    let _ = sender.try_send(TrayMessage::Enable);
                }
            }

            let _ = grin_gui.config.save();
        }
        Message::Interaction(Interaction::OpenLink(link)) => {
            log::debug!("Interaction::OpenLink({})", &link);

            return Ok(Command::perform(
                async {
                    let _ = opener::open(link);
                },
                Message::None,
            ));
        }
        // Application shutdown
        Message::Interaction(Interaction::Exit) => {
            grin_gui.safe_exit();
        }
        Message::Interaction(Interaction::ExitCancel) => {
            grin_gui.show_exit(false);
        }
        Message::Interaction(_) => {}
        Message::RuntimeEvent(_) => {}
        Message::None(_) => {}
    }

    Ok(Command::none())
}

#[cfg(not(target_os = "linux"))]
async fn select_directory() -> Option<PathBuf> {
    use rfd::AsyncFileDialog;

    let dialog = AsyncFileDialog::new();
    if let Some(show) = dialog.pick_folder().await {
        return Some(show.path().to_path_buf());
    }

    None
}

#[cfg(target_os = "linux")]
async fn select_directory() -> Option<PathBuf> {
    use native_dialog::FileDialog;

    let dialog = FileDialog::new();
    if let Ok(Some(show)) = dialog.show_open_single_dir() {
        return Some(show);
    }

    None
}

/// Hardcoded binary names for each compilation target
/// that gets published to the Github Release
const fn bin_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "grin-gui.exe"
    }

    #[cfg(target_os = "macos")]
    {
        "grin-gui"
    }

    #[cfg(target_os = "linux")]
    {
        "grin-gui.AppImage"
    }
}
