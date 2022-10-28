use {
    super::super::super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, GrinGui, Interaction, Message},
    crate::localization::localized_string,
    crate::Result,
    grin_gui_core::config::Config,
    grin_gui_core::{
        theme::ColorPalette,
        wallet::{create_grin_wallet_path, ChainTypes},
    },
    iced::{
        alignment, button, scrollable, Alignment, Button, Checkbox, Column, Command, Container,
        Element, Length, Row, Scrollable, Space, Text,
    },
    native_dialog::FileDialog,
    std::path::PathBuf,
};

use grin_gui_widgets::{table_row::StyleSheet, TableRow};
use isahc::head;

use crate::gui::element::DEFAULT_SUB_HEADER_FONT_SIZE;

pub struct StateContainer {
    pub back_button_state: button::State,
    selected_wallet_index: usize,
    load_wallet_button_state: button::State,
    select_folder_button_state: button::State,
    new_wallet_button_state: button::State,
    scrollable_state: scrollable::State,
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
            back_button_state: Default::default(),
            selected_wallet_index: 0,
            load_wallet_button_state: Default::default(),
            select_folder_button_state: Default::default(),
            new_wallet_button_state: Default::default(),
            scrollable_state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocalViewInteraction {
    Back,
    WalletRowSelect(bool, usize),
    LoadWallet(usize),
    LocateWallet,
    CreateWallet,
}

pub fn handle_message<'a>(
    grin_gui: &mut GrinGui,
    message: LocalViewInteraction,
) -> Result<Command<Message>> {
    match message {
        LocalViewInteraction::Back => {
            grin_gui.wallet_state.setup_state.mode = super::Mode::Init;
        }
        LocalViewInteraction::WalletRowSelect(is_selected, index) => {
            if is_selected {
                grin_gui
                    .wallet_state
                    .setup_state
                    .setup_wallet_list_state
                    .selected_wallet_index = index;
            }
        }
        LocalViewInteraction::LoadWallet(index) => {
            grin_gui.config.current_wallet_index = Some(index);
            grin_gui.wallet_state.mode = crate::gui::element::wallet::Mode::Operation;
        }
        LocalViewInteraction::LocateWallet => {
            match FileDialog::new().show_open_single_file() {
                Ok(path) => {
                    match path {
                        Some(d) => {
                            match validate_directory(d) {
                                Ok(wallet_was_imported) => {}
                                Err(err) => {
                                    // tell the user why this directory failed
                                }
                            }
                        }
                        None => {}
                    }
                }
                Err(e) => {
                    log::debug!("wallet_list.rs::LocalViewInteraction::LocateWallet {}", e);
                }
            };
        }
        LocalViewInteraction::CreateWallet => {
            let state = &mut grin_gui.wallet_state.setup_state;
            let config = &grin_gui.config;
            let wallet_default_name = localized_string("wallet-default-name");
            let mut wallet_display_name = wallet_default_name.clone();
            let mut i = 1;

            // wallet display name must be unique: i.e. Default 1, Default 2, ...
            while let Some(_) = config
                .wallets
                .iter()
                .find(|wallet| wallet.display_name == wallet_display_name)
            {
                wallet_display_name = format!("{} {}", wallet_default_name, i);
                i += 1;
            }

            // i.e. default_1, default_2, ...
            let wallet_dir: String = str::replace(&wallet_display_name.to_lowercase(), " ", "_");

            state
                .setup_wallet_state
                .advanced_options_state
                .top_level_directory = create_grin_wallet_path(&ChainTypes::Mainnet, &wallet_dir);

            grin_gui.wallet_state.mode =
                crate::gui::element::wallet::Mode::CreateWallet(wallet_display_name);
        }
    }

    Ok(Command::none())
}

struct DirectoryValidationError;

fn validate_directory(_d: PathBuf) -> Result<bool, DirectoryValidationError> {
    Ok(true)
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    state: &'a mut StateContainer,
    config: &Config,
) -> Container<'a, Message> {
    let paddy = DEFAULT_PADDING + 5;
    let button_height = Length::Units(20); 

    let title = Text::new(localized_string("wallet-list")).size(DEFAULT_HEADER_FONT_SIZE);
    let title_container =
        Container::new(title).style(style::BrightBackgroundContainer(color_palette));

    let new_wallet_container =
        Container::new(Text::new(localized_string("create-wallet")).size(DEFAULT_FONT_SIZE))
            .height(button_height)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let new_wallet_button: Element<Interaction> =
        Button::new(&mut state.new_wallet_button_state, new_wallet_container)
            .style(style::DefaultButton(color_palette))
            .on_press(Interaction::WalletListWalletViewInteraction(
                LocalViewInteraction::CreateWallet,
            ))
            .into();

    // add additional buttons here
    let button_row = Row::new().push(new_wallet_button.map(Message::Interaction));

    let segmented_mode_container =
        Container::new(button_row).style(style::SegmentedContainer(color_palette));

    let header_row = Row::new()
        .height(Length::Units(50))
        .push(title_container)
        .push(Space::with_width(Length::Fill))
        .push(segmented_mode_container)
        .align_items(Alignment::Center);

    let header_container = Container::new(header_row).padding(iced::Padding::from([
        0,         // top
        paddy,     // right
        0,         // bottom
        paddy + 5, // left
    ]));

    let name_header = Text::new(localized_string("name")).size(DEFAULT_SUB_HEADER_FONT_SIZE);
    let name_header_container =
        Container::new(name_header).style(style::FadedBrightForegroundContainer(color_palette));

    let chain_header = Text::new(localized_string("type")).size(DEFAULT_SUB_HEADER_FONT_SIZE);
    let chain_header_container =
        Container::new(chain_header).style(style::FadedBrightForegroundContainer(color_palette));

    let directory_header = Text::new(localized_string("folder")).size(DEFAULT_SUB_HEADER_FONT_SIZE);
    let directory_header_container = Container::new(directory_header)
        .style(style::FadedBrightForegroundContainer(color_palette));

    let table_header_row = Row::new()
        .push(
            Column::new()
                .push(name_header_container)
                .width(Length::FillPortion(1)),
        )
        .push(
            Column::new()
                .push(chain_header_container)
                .width(Length::FillPortion(1)),
        )
        .push(
            Column::new()
                .push(directory_header_container)
                .width(Length::FillPortion(3)),
        );

    let table_header_container = Container::new(table_header_row)
        .padding(iced::Padding::from([
            9,          // top
            paddy +  9, // right 
            9,          // bottom
            9,          // left
        ]))
        .style(style::PanelForeground(color_palette));

    let mut wallet_rows: Vec<_> = vec![];
    for (pos, w) in config.wallets.iter().enumerate() {
        // si quieres el checkbox
        // let checkbox = Checkbox::new(state.selected_wallet_index == pos, "", move |b| {
        //     Message::Interaction(Interaction::WalletListWalletViewInteraction(
        //         LocalViewInteraction::WalletRowSelect(b, pos),
        //     ))
        // })
        // .style(style::DefaultCheckbox(color_palette))
        // .text_size(DEFAULT_FONT_SIZE)
        // .spacing(10);

        let selected_wallet = state.selected_wallet_index == pos;
        let wallet_name = Text::new(w.display_name.clone()).size(DEFAULT_FONT_SIZE);
        let chain_name = Text::new(w.chain_type.shortname()).size(DEFAULT_FONT_SIZE);

        let mut wallet_name_container =
            Container::new(wallet_name).style(style::HoverableForegroundContainer(color_palette));
        
        let mut wallet_chain_container =
            Container::new(chain_name).style(style::HoverableForegroundContainer(color_palette));

        let tld_string = match &w.tld {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::from("Unknown"),
        };
        let wallet_directory = Text::new(tld_string).size(DEFAULT_FONT_SIZE);

        let mut wallet_directory_container = Container::new(wallet_directory)
            .style(style::HoverableForegroundContainer(color_palette));

        if selected_wallet {
            wallet_name_container = wallet_name_container.style(style::HoverableBrightForegroundContainer(color_palette));
            wallet_chain_container = wallet_chain_container.style(style::HoverableBrightForegroundContainer(color_palette));
            wallet_directory_container = wallet_directory_container.style(style::HoverableBrightForegroundContainer(color_palette));
        }

        let wallet_row = Row::new()
            //.push(checkbox)
            .push(
                Column::new()
                    .push(wallet_name_container)
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(wallet_chain_container)
                    .width(Length::FillPortion(1)),
            )
            .push(
                Column::new()
                    .push(wallet_directory_container)
                    .width(Length::FillPortion(3)),
            );

        let mut table_row = TableRow::new(wallet_row)
            .padding(iced::Padding::from(9))
            .width(Length::Fill)
            .on_press(move |_| {
                log::debug!("data_container::table_row::on_press {}", pos);

                Message::Interaction(Interaction::WalletListWalletViewInteraction(
                    LocalViewInteraction::WalletRowSelect(true, pos),
                ))
            });

        if selected_wallet {
            // selected wallet should be highlighted
            table_row = table_row.style(style::TableRowSelected(color_palette));
        } else {
            // contrast row styles to spice things up
            if pos % 2 == 0 {
                table_row = table_row.style(style::TableRowLowlife(color_palette));
            } else {
                table_row = table_row.style(style::TableRowHighlife(color_palette));
            }
        }

        wallet_rows.push(table_row.into());
    }

    let wallet_column = Column::new()
        .push(Column::with_children(wallet_rows))
        .padding(iced::Padding::from([0, paddy, 0, 0]));

    let load_wallet_button_container =
        Container::new(Text::new(localized_string("load-wallet")).size(DEFAULT_FONT_SIZE))
            .height(button_height)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let mut load_wallet_button = Button::new(
        &mut state.load_wallet_button_state,
        load_wallet_button_container,
    )
    .style(style::DefaultBoxedButton(color_palette));

    // the load wallet button should be disabled if there are no wallets
    if !config.wallets.is_empty() {
        load_wallet_button = load_wallet_button.on_press(Interaction::WalletListWalletViewInteraction(
            LocalViewInteraction::LoadWallet(state.selected_wallet_index),
        ))
    }
 
    let load_wallet_button: Element<Interaction> = load_wallet_button.into();

    let select_folder_button_container =
        Container::new(Text::new(localized_string("select-other")).size(DEFAULT_FONT_SIZE))
            .height(button_height)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

    let select_other_button: Element<Interaction> = Button::new(
        &mut state.select_folder_button_state,
        select_folder_button_container,
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::WalletListWalletViewInteraction(
        LocalViewInteraction::LocateWallet,
    ))
    .into();

    let button_row = Row::new()
        .push(load_wallet_button.map(Message::Interaction))
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(select_other_button.map(Message::Interaction))
        .height(Length::Shrink)
        .padding(iced::Padding::from([paddy, 0, 0, 0]));

    let scrollable = Scrollable::new(&mut state.scrollable_state)
        .push(wallet_column)
        .style(style::Scrollable(color_palette));

    let table_colummn = Column::new().push(table_header_container).push(scrollable);
    let table_container = Container::new(table_colummn)
        .style(style::PanelBordered(color_palette))
        .height(Length::Fill)
        .padding(1);

    let row = Row::new()
        .push(Column::new().push(table_container).push(button_row))
        .padding(iced::Padding::from([0, paddy, paddy, paddy]));

    let content = Container::new(row)
        .center_x()
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(style::NormalBackgroundContainer(color_palette));

    let wrapper_column = Column::new()
        .height(Length::Fill)
        .push(header_container)
        .push(content);

    // Returns the final container.
    Container::new(wrapper_column)
}
