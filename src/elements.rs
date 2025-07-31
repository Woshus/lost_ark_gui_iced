use crate::home;
use crate::styles;
use iced::widget::{Button, Column, Image, Row, Text, TextInput, button, image};
use iced::{Alignment, Element, Length, Theme};
use std::collections::HashMap;
use std::path::PathBuf;
pub const MY_FONT: iced::Font = iced::Font::with_name("Cleanow");

// Base Great Success Rate is 5%

pub fn get_menubar() -> Element<'static, home::Message> {
    let crafting_menu_button = Button::new(Text::new("Crafting").font(MY_FONT))
        .on_press(home::Message::CraftingMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    let todo_menu_button1 = Button::new(Text::new("TODO").font(MY_FONT))
        .on_press(home::Message::TODOMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    let todo_menu_button2 = Button::new(Text::new("TODO").font(MY_FONT))
        .on_press(home::Message::TODOMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    let todo_menu_button3 = Button::new(Text::new("TODO").font(MY_FONT))
        .on_press(home::Message::TODOMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));
    let todo_menu_button4 = Button::new(Text::new("TODO").font(MY_FONT))
        .on_press(home::Message::TODOMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));
    let todo_menu_button5 = Button::new(Text::new("TODO").font(MY_FONT))
        .on_press(home::Message::TODOMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));
    let todo_menu_button6 = Button::new(Text::new("TODO").font(MY_FONT))
        .on_press(home::Message::TODOMenuClicked)
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    Row::new()
        .spacing(10)
        .padding(10)
        .push(crafting_menu_button)
        .push(todo_menu_button1)
        .push(todo_menu_button2)
        .push(todo_menu_button3)
        .push(todo_menu_button4)
        .push(todo_menu_button5)
        .push(todo_menu_button6)
        .into()
}
pub fn get_table_selection(menu_type: &home::MenuType) -> Element<'static, home::Message> {
    let materials_button = Button::new(Text::new("Materials").font(MY_FONT))
        .on_press(home::Message::TableSelectionClicked(
            home::MenuType::Materials,
        ))
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    let items_button = Button::new(Text::new("Items").font(MY_FONT))
        .on_press(home::Message::TableSelectionClicked(home::MenuType::Items))
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    let settings_button = Button::new(Text::new("Settings").font(MY_FONT))
        .on_press(home::Message::TableSelectionClicked(
            home::MenuType::Settings,
        ))
        .style(move |theme: &Theme, status: button::Status| styles::clear_button(theme, status))
        .width(Length::FillPortion(1));

    Row::new()
        .push(materials_button)
        .push(items_button)
        .push(settings_button)
        .into()
}
pub fn price_entry<P: Into<PathBuf>>(
    path: P,
    mat_costs: &HashMap<String, String>,
    key: String,
) -> Element<home::Message> {
    let display_value = mat_costs.get(&key);
    let handle = image::Handle::from_path(path);
    let image = Image::new(handle)
        .width(Length::FillPortion(1))
        .height(Length::FillPortion(1));

    let text_input = TextInput::new("0", display_value.unwrap_or(&String::new()))
        .on_input(move |s| home::Message::MatPriceChanged(key.clone(), s));

    Row::new()
        .push(image)
        .push(text_input)
        .align_y(Alignment::Center)
        .into()
}
pub fn crafting_entry<F, P>(
    mat_img_path: P,
    item_img_path: P,
    cost1: f32,
    cost2: f32,
    cost3: f32,
    rev1: f32,
    amt: f32,
    formula: F,
) -> Element<'static, home::Message>
where
    F: Fn(f32, f32, f32) -> f32,
    P: Into<PathBuf>,
{
    let mat_handle = image::Handle::from_path(mat_img_path);
    let item_handle = image::Handle::from_path(item_img_path);
    let mat_image = Image::new(mat_handle)
        .height(Length::FillPortion(1));
    let item_image = Image::new(item_handle)
        .height(Length::FillPortion(1));

    let item_row = Row::new()
        .push(mat_image)
        .push(item_image)
        .width(Length::FillPortion(1))
        .height(Length::FillPortion(1))
        .spacing(10);

    let cost = formula(cost1, cost2, cost3);
    // 5% market fee = (0.95*revenue).floor(). Floor because market rounds per unit, so an example 10*.95 9.5, but the market fee would be 1, therefore 9.
    // 1.05 is the default crafting great success chance.
    let profit = (0.95 * rev1).floor() * amt * 1.05 - cost;
    let cost_text = Text::new(format!("{:.2}", cost)).width(Length::FillPortion(1));
    let profit_text = Text::new(format!("{:.2}", profit)).width(Length::FillPortion(1));
    let full_text = Text::new(format!("{:.2}", profit * 40.0)).width(Length::FillPortion(1));
    Row::new()
        .push(item_row)
        .push(cost_text)
        .push(profit_text)
        .push(full_text)
        .into()
}
pub fn get_crafted_cost_table(mat_costs: &HashMap<String, String>) -> Element<home::Message> {
    let handle = image::Handle::from_path("src/assets/images/ferris.png");
    let ofm_price_entry = price_entry(
        "src/assets/images/ofm.png",
        mat_costs,
        "ofm_price".to_string(),
    );
    let sofm_price_entry = price_entry(
        "src/assets/images/sofm.png",
        mat_costs,
        "sofm_price".to_string(),
    );
    let pofm_price_entry = price_entry(
        "src/assets/images/pofm.png",
        mat_costs,
        "pofm_price".to_string(),
    );
    let afm_price_entry = price_entry(
        "src/assets/images/afm.png",
        mat_costs,
        "afm_price".to_string(),
    );

    let fusion_mat_col = Column::new()
        .push(ofm_price_entry)
        .push(sofm_price_entry)
        .push(pofm_price_entry)
        .push(afm_price_entry)
        .spacing(10)
        .padding(10);

    let mhpp_price_entry = price_entry(
        "src/assets/images/mhpp.png",
        mat_costs,
        "mhpp_price".to_string(),
    );
    let ehpp_price_entry = price_entry(
        "src/assets/images/ehpp.png",
        mat_costs,
        "ehpp_price".to_string(),
    );
    let sehpp_price_entry = price_entry(
        "src/assets/images/sehpp.png",
        mat_costs,
        "sehpp_price".to_string(),
    );
    let hp_potion_col = Column::new()
        .push(mhpp_price_entry)
        .push(ehpp_price_entry)
        .push(sehpp_price_entry)
        .spacing(10)
        .padding(10);

    Row::new().push(fusion_mat_col).push(hp_potion_col).into()
}
pub fn get_mat_cost_table(mat_costs: &HashMap<String, String>) -> Element<home::Message> {
    let clog_price_entry = price_entry(
        "src/assets/images/clog.png",
        mat_costs,
        "clog_price".to_string(),
    );
    let ulog_price_entry = price_entry(
        "src/assets/images/ulog.png",
        mat_costs,
        "ulog_price".to_string(),
    );
    let olog_price_entry = price_entry(
        "src/assets/images/olog.png",
        mat_costs,
        "olog_price".to_string(),
    );
    let alog_price_entry = price_entry(
        "src/assets/images/alog.png",
        mat_costs,
        "alog_price".to_string(),
    );

    let log_mat_col = Column::new()
        .push(clog_price_entry)
        .push(ulog_price_entry)
        .push(olog_price_entry)
        .push(alog_price_entry)
        .spacing(10)
        .padding(10);

    let cfish_price_entry = price_entry(
        "src/assets/images/cfish.png",
        mat_costs,
        "cfish_price".to_string(),
    );
    let ufish_price_entry = price_entry(
        "src/assets/images/ufish.png",
        mat_costs,
        "ufish_price".to_string(),
    );
    let ofish_price_entry = price_entry(
        "src/assets/images/ofish.png",
        mat_costs,
        "ofish_price".to_string(),
    );
    let afish_price_entry = price_entry(
        "src/assets/images/afish.png",
        mat_costs,
        "afish_price".to_string(),
    );

    let fish_mat_col = Column::new()
        .push(cfish_price_entry)
        .push(ufish_price_entry)
        .push(ofish_price_entry)
        .push(afish_price_entry)
        .spacing(10)
        .padding(10);

    let crelic_price_entry = price_entry(
        "src/assets/images/crelic.png",
        mat_costs,
        "crelic_price".to_string(),
    );
    let urelic_price_entry = price_entry(
        "src/assets/images/urelic.png",
        mat_costs,
        "urelic_price".to_string(),
    );
    let orelic_price_entry = price_entry(
        "src/assets/images/orelic.png",
        mat_costs,
        "orelic_price".to_string(),
    );
    let arelic_price_entry = price_entry(
        "src/assets/images/arelic.png",
        mat_costs,
        "arelic_price".to_string(),
    );

    let relic_mat_col = Column::new()
        .push(crelic_price_entry)
        .push(urelic_price_entry)
        .push(orelic_price_entry)
        .push(arelic_price_entry)
        .spacing(10)
        .padding(10);

    let cflow_price_entry = price_entry(
        "src/assets/images/cflow.png",
        mat_costs,
        "cflow_price".to_string(),
    );
    let uflow_price_entry = price_entry(
        "src/assets/images/uflow.png",
        mat_costs,
        "uflow_price".to_string(),
    );
    let oflow_price_entry = price_entry(
        "src/assets/images/oflow.png",
        mat_costs,
        "oflow_price".to_string(),
    );
    let aflow_price_entry = price_entry(
        "src/assets/images/aflow.png",
        mat_costs,
        "aflow_price".to_string(),
    );

    let flow_mat_col = Column::new()
        .push(cflow_price_entry)
        .push(uflow_price_entry)
        .push(oflow_price_entry)
        .push(aflow_price_entry)
        .spacing(10)
        .padding(10);

    let cmeat_price_entry = price_entry(
        "src/assets/images/cmeat.png",
        mat_costs,
        "cmeat_price".to_string(),
    );
    let umeat_price_entry = price_entry(
        "src/assets/images/umeat.png",
        mat_costs,
        "umeat_price".to_string(),
    );
    let omeat_price_entry = price_entry(
        "src/assets/images/omeat.png",
        mat_costs,
        "omeat_price".to_string(),
    );
    let ameat_price_entry = price_entry(
        "src/assets/images/ameat.png",
        mat_costs,
        "ameat_price".to_string(),
    );

    let meat_mat_col = Column::new()
        .push(cmeat_price_entry)
        .push(umeat_price_entry)
        .push(omeat_price_entry)
        .push(ameat_price_entry)
        .spacing(10)
        .padding(10);

    let core_price_entry = price_entry(
        "src/assets/images/core.png",
        mat_costs,
        "core_price".to_string(),
    );
    let uore_price_entry = price_entry(
        "src/assets/images/uore.png",
        mat_costs,
        "uore_price".to_string(),
    );
    let oore_price_entry = price_entry(
        "src/assets/images/oore.png",
        mat_costs,
        "oore_price".to_string(),
    );
    let aore_price_entry = price_entry(
        "src/assets/images/aore.png",
        mat_costs,
        "aore_price".to_string(),
    );

    let ore_mat_col = Column::new()
        .push(core_price_entry)
        .push(uore_price_entry)
        .push(oore_price_entry)
        .push(aore_price_entry)
        .spacing(10)
        .padding(10);

    Row::new()
        .push(log_mat_col)
        .push(fish_mat_col)
        .push(relic_mat_col)
        .push(flow_mat_col)
        .push(meat_mat_col)
        .push(ore_mat_col)
        .into()
}
pub fn get_crafting_cost_table(mat_costs: &HashMap<String, String>) -> Element<home::Message> {
    // Create Table Column Names
    let item_text = Text::new("ITEM")
        .width(Length::FillPortion(1))
        .font(MY_FONT);
    let cost_text = Text::new("COST")
        .width(Length::FillPortion(1))
        .font(MY_FONT);
    let profit_text = Text::new("PROFIT")
        .width(Length::FillPortion(1))
        .font(MY_FONT);
    let full_text = Text::new("FULL")
        .width(Length::FillPortion(1))
        .font(MY_FONT);
    let crafting_table_header = Row::new()
        .push(item_text)
        .push(cost_text)
        .push(profit_text)
        .push(full_text);

    let formula = |a: f32, b: f32, c: f32| 400.0 + 0.86 * a + 0.45 * b + 0.33 * c;

    let log_entry = crafting_entry(
        "src/assets/images/afm.png",
        "src/assets/images/alog.png",
        mat_costs
            .get("clog_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("ulog_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("alog_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afm_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        10.0,
        formula,
    );
    let fish_entry = crafting_entry(
        "src/assets/images/afm.png",
        "src/assets/images/afish.png",
        mat_costs
            .get("cfish_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("ufish_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afish_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afm_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        10.0,
        formula,
    );
    let relic_entry = crafting_entry(
        "src/assets/images/afm.png",
        "src/assets/images/arelic.png",
        mat_costs
            .get("crelic_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("urelic_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("arelic_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afm_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        10.0,
        formula,
    );
    let flow_entry = crafting_entry(
        "src/assets/images/afm.png",
        "src/assets/images/aflow.png",
        mat_costs
            .get("cflow_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("uflow_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("aflow_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afm_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        10.0,
        formula,
    );
    let meat_entry = crafting_entry(
        "src/assets/images/afm.png",
        "src/assets/images/ameat.png",
        mat_costs
            .get("cmeat_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("umeat_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("ameat_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afm_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        10.0,
        formula,
    );
    let ore_entry = crafting_entry(
        "src/assets/images/afm.png",
        "src/assets/images/aore.png",
        mat_costs
            .get("core_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("uore_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("aore_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        mat_costs
            .get("afm_price")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        10.0,
        formula,
    );
    let crafting_cost_table = Column::new()
        .push(crafting_table_header)
        .push(log_entry)
        .push(relic_entry)
        .push(fish_entry)
        .push(flow_entry)
        .push(meat_entry)
        .push(ore_entry);
    crafting_cost_table.into()
}
