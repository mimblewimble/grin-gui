use {
    super::{DEFAULT_FONT_SIZE, DEFAULT_HEADER_FONT_SIZE, DEFAULT_PADDING},
    crate::gui::{style, Interaction, Message},
    crate::localization::localized_string,
    grin_gui_core::{theme::ColorPalette, utility::Release},
    iced::{alignment, Alignment, Command, Element, Length},
    iced::widget::{
        button, pick_list, scrollable, text_input, Button, Checkbox, Column, Container, PickList,
        Row, Scrollable, Space, Text, TextInput,
    },
    std::collections::HashMap,
    strfmt::strfmt,
};

pub struct StateContainer {
}

impl Default for StateContainer {
    fn default() -> Self {
        Self {
        }
    }
}

pub fn data_container<'a>(
    color_palette: ColorPalette,
    release: &Option<Release>,
    state: &'a mut StateContainer,
) -> Container<'a, Message> {
    let grin_gui_title = Text::new(localized_string("grin")).size(DEFAULT_HEADER_FONT_SIZE);
    let grin_gui_title_container =
        Container::new(grin_gui_title).style(style::BrightBackgroundContainer(color_palette));

    let changelog_title_text = Text::new(if let Some(release) = release {
        let mut vars = HashMap::new();
        // TODO (casperstorm): change "addon" to "tag" or "version".
        vars.insert("addon".to_string(), &release.tag_name);
        let fmt = localized_string("changelog-for");
        strfmt(&fmt, &vars).unwrap()
    } else {
        localized_string("changelog")
    })
    .size(DEFAULT_FONT_SIZE);

    let changelog_text = Text::new(if let Some(release) = release {
        release.body.clone()
    } else {
        localized_string("no-changelog")
    })
    .size(DEFAULT_FONT_SIZE);

    let website_button: Element<Interaction> = Button::new(
        Text::new(localized_string("website")).size(DEFAULT_FONT_SIZE),
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::OpenLink(localized_string("website-http")))
    .into();

    let donation_button: Element<Interaction> = Button::new(
        Text::new(localized_string("donate")).size(DEFAULT_FONT_SIZE),
    )
    .style(style::DefaultBoxedButton(color_palette))
    .on_press(Interaction::OpenLink(localized_string("donate-http")))
    .into();

    let button_row = Row::new()
        .spacing(DEFAULT_PADDING)
        .push(website_button.map(Message::Interaction))
        .push(donation_button.map(Message::Interaction));

    let mut scrollable = Scrollable::new(&mut state.scrollable)
        .spacing(1)
        .height(Length::FillPortion(1))
        .style(style::Scrollable(color_palette));

    let changelog_text_container =
        Container::new(changelog_text).style(style::NormalBackgroundContainer(color_palette));
    let changelog_title_container =
        Container::new(changelog_title_text).style(style::BrightBackgroundContainer(color_palette));

    scrollable = scrollable
        .push(grin_gui_title_container)
        .push(Space::new(Length::Units(0), Length::Units(DEFAULT_PADDING)))
        .push(button_row)
        .push(Space::new(Length::Units(0), Length::Units(DEFAULT_PADDING)))
        .push(changelog_title_container)
        .push(Space::new(Length::Units(0), Length::Units(5)))
        .push(changelog_text_container);

    let col = Column::new().push(scrollable);
    let row = Row::new()
        .push(Space::new(Length::Units(DEFAULT_PADDING), Length::Units(0)))
        .push(col);

    // Returns the final container.
    Container::new(row)
        .center_x()
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(style::NormalBackgroundContainer(color_palette))
        .padding(20)
}
