use iced::{Element, Task, overlay::menu};
use std::{arch::x86_64, collections::HashMap};
mod action;
mod counter;
mod elements;
mod hello;
mod home;
mod styles;

pub use action::Action;
pub use counter::Counter;

//TODO: Add a txt file or something to pull from to instantiate values for stuff like mat_cost_table.

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .window_size(iced::Size::new(800.0, 600.0))
        .theme(App::theme)
        .font(include_bytes!("assets/fonts/Cleanow.ttf").as_slice())
        .centered()
        .run_with(App::new)
}

#[derive(Debug, Clone)]
enum Screen {
    Counter,
    Hello,
    Home,
}

#[derive(Debug, Clone)]
enum Message {
    Hello(hello::Message),
    Counter(counter::Message),
    Home(home::Message),
}

struct App {
    mat_costs: HashMap<String, String>,
    counter: Counter,
    screen: Screen,
    crafting_menu_type: home::MenuType,
}

impl App {
    fn theme(&self) -> iced::Theme {
        iced::Theme::TokyoNightLight
    }

    fn title(&self) -> String {
        "Lost Ark App".to_string()
    }
    fn new() -> (Self, Task<Message>) {
        (
            App {
                mat_costs: HashMap::new(),
                counter: Counter::new(0),
                screen: Screen::Home,
                crafting_menu_type: home::MenuType::Materials,
            },
            Task::none(),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Counter(counter::Message::SwapPage) => {
                self.screen = Screen::Hello;
                Task::none()
            }
            Message::Hello(hello::Message::SwapPage) => {
                self.screen = Screen::Counter;
                Task::none()
            }
            Message::Counter(msg) => {
                counter::update(&mut self.counter, msg);
                Task::none()
            }

            Message::Home(msg) => {
                match msg {
                    home::Message::CraftingMenuClicked => Task::none(),
                    home::Message::MatPriceChanged(_, _) => {
                        home::update(&mut self.mat_costs, msg).map(move |m| Message::Home(m));
                        Task::none()
                    }
                    home::Message::TODOMenuClicked => Task::none(),
                    home::Message::TableSelectionClicked(menu_type) => {
                        self.crafting_menu_type = menu_type;
                        Task::none()
                    }
                }
                //     let action = home::update(&mut self.mat_costs, msg).map(move |m| Message::Home(m));
                //     // if let Some(instruction) = action.instruction {
                //     //     Task::none()
                //     // } else {
                //     //     Task::none()
                //     // }
                //     Task::none()
                // }
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Home => home::view(&self).map(Message::Home),
            Screen::Counter => counter::view(&self.counter).map(Message::Counter),
            Screen::Hello => hello::view().map(Message::Hello),
        }
    }
}
