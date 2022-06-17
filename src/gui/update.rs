use {
    super::{Ajour, Interaction, Message, Mode},
    crate::{log_error, Result},
    ajour_core::fs::PersistentData,
    //ajour_widgets::header::ResizeEvent,
    std::path::PathBuf,
    iced::Command,
};

#[cfg(target_os = "windows")]
use crate::tray::{TrayMessage, SHOULD_EXIT, TRAY_SENDER};
#[cfg(target_os = "windows")]
use std::sync::atomic::Ordering;

pub fn handle_message(ajour: &mut Ajour, message: Message) -> Result<Command<Message>> {
    match message {
        Message::Interaction(Interaction::ModeSelected(mode)) => {
            log::debug!("Interaction::ModeSelected({:?})", mode);
            // Set Mode
            ajour.mode = mode;
        }
        Message::Interaction(Interaction::ModeSelectedSettings(mode)) => {
            log::debug!("Interaction::ModeSelectedSettings({:?})", mode);
            // Set Mode
            ajour.settings_state.mode = mode;
        }
        Message::Error(error) => {
            log_error(&error);
            ajour.error = Some(error);
        }
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::Resized { width, height },
        )) => {
            let width = (width as f64 * ajour.scale_state.scale) as u32;
            let height = (height as f64 * ajour.scale_state.scale) as u32;

            // Minimizing Ajour on Windows will call this function with 0, 0.
            // We don't want to save that in config, because then it will start with zero size.
            if width > 0 && height > 0 {
                ajour.config.window_size = Some((width, height));
                let _ = ajour.config.save();
            }
        }
        #[cfg(target_os = "windows")]
        Message::RuntimeEvent(iced_native::Event::Window(
            iced_native::window::Event::CloseRequested,
        )) => {
            log::debug!("Message::RuntimeEvent(CloseRequested)");

            if let Some(sender) = TRAY_SENDER.get() {
                if ajour.config.close_to_tray {
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
            if !ajour.config.is_keybindings_enabled
                || modifiers != iced::keyboard::Modifiers::default()
            {
                return Ok(Command::none());
            }

            match key_code {
                iced::keyboard::KeyCode::A => {
                    /*let flavor = ajour.config.wow.flavor;
                    ajour.mode = Mode::MyAddons(flavor);*/
                }
                iced::keyboard::KeyCode::C => {
                    ajour.mode = Mode::Catalog;
                }
                iced::keyboard::KeyCode::R => {
                    /*let mode = ajour.mode.clone();
                    return handle_message(ajour, Message::Interaction(Interaction::Refresh(mode)));*/
                }
                iced::keyboard::KeyCode::S => {
                    ajour.mode = Mode::Settings;
                }
                iced::keyboard::KeyCode::U => {
                    /*let mode = ajour.mode.clone();
                    return handle_message(
                        ajour,
                        Message::Interaction(Interaction::UpdateAll(mode)),
                    );*/
                }
                iced::keyboard::KeyCode::W => {
                    /*let flavor = ajour.config.wow.flavor;
                    ajour.mode = Mode::MyWeakAuras(flavor);*/
                }
                iced::keyboard::KeyCode::I => {
                    ajour.mode = Mode::Install;
                }
                iced::keyboard::KeyCode::Escape => match ajour.mode {
                    /*Mode::Settings | Mode::About => {
                        ajour.mode = Mode::MyAddons(ajour.config.wow.flavor);
                    }
                    Mode::MyAddons(_) => {
                        ajour.addons_search_state.query = None;
                    }
                    Mode::Catalog => {
                        ajour.catalog_search_state.query = None;
                    }*/
                    _ => (),
                },
                _ => (),
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleCloseToTray(enable)) => {
            log::debug!("Interaction::ToggleCloseToTray({})", enable);

            ajour.config.close_to_tray = enable;

            // Remove start closed to tray if we are disabling
            if !enable {
                ajour.config.start_closed_to_tray = false;
            }

            let _ = ajour.config.save();

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

            ajour.config.autostart = enable;

            let _ = ajour.config.save();

            if let Some(sender) = TRAY_SENDER.get() {
                let _ = sender.try_send(TrayMessage::ToggleAutoStart(enable));
            }
        }
        #[cfg(target_os = "windows")]
        Message::Interaction(Interaction::ToggleStartClosedToTray(enable)) => {
            log::debug!("Interaction::ToggleStartClosedToTray({})", enable);

            ajour.config.start_closed_to_tray = enable;

            // Enable tray if this feature is enabled
            if enable && !ajour.config.close_to_tray {
                ajour.config.close_to_tray = true;

                if let Some(sender) = TRAY_SENDER.get() {
                    let _ = sender.try_send(TrayMessage::Enable);
                }
            }

            let _ = ajour.config.save();
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
        "ajour.exe"
    }

    #[cfg(target_os = "macos")]
    {
        "ajour"
    }

    #[cfg(target_os = "linux")]
    {
        "ajour.AppImage"
    }
}
