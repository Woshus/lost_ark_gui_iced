use iced::widget::{Button, Column, Container, Image, Row, Text, button, image};
use iced::{Color, Element, Length, Renderer, Task, Theme};
pub const MY_FONT: iced::Font = iced::Font::with_name("Cleanow");
use crate::Action;
use std::collections::HashMap;

use crate::App;
use crate::elements;

#[derive(Debug, Clone)]
pub enum Message {
    TODOMenuClicked,
    CraftingMenuClicked,
    MatPriceChanged(String, String),
    TableSelectionClicked(MenuType),
}

#[derive(Debug, Clone)]
pub enum MenuType{
    Materials,
    Items,
    Settings
}

pub enum Instruction {
    Back,
    Save,
    StartEdit,
    Cancel,
}

pub fn view(app: &App) -> Element<Message> {
    // Top Menu Buttons (Currently Holds Different Toolnames)
    let menubar = elements::get_menubar();
    let table_selection = elements::get_table_selection(&app.crafting_menu_type);
    // Material Price Table, holds the costs of all materials used in later calculations
    let table = match &app.crafting_menu_type {
        MenuType::Materials => elements::get_mat_cost_table(&app.mat_costs),
        MenuType::Items => elements::get_crafted_cost_table(&app.mat_costs),
        MenuType::Settings => Row::new().into()
    };

    let crafting_cost_table = elements::get_crafting_cost_table(&app.mat_costs);

    let content = Column::new()
        .push(menubar)
        .push(table_selection)
        .push(table)
        .push(crafting_cost_table);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn update(
    mat_costs: &mut HashMap<String, String>,
    message: Message,
) -> Action<Instruction, Message> {
    match message {
        Message::MatPriceChanged(key, value) => {
            if value.parse::<u32>().is_ok() || value.is_empty() {
                mat_costs.insert(key, value);
            }
            Action::none()
        }
        _ => Action::none(),
    }
}
