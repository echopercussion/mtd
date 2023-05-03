use rainfrog::debug::RainfrogDebug;
use yew::prelude::*;
use rainfrog::table::{TableItemDisplay};
use rainfrog::dropdown::{DropdownItemDisplay};
use rainfrog::multi_select_dropdown::{MultiSelectDropdownItemDisplay, ListLayout};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameMode {
    Control,
    Hybrid,
    Payload,
    Push,
}

impl GameMode {
    pub fn to_string(&self) -> &str {
        match self {
            GameMode::Control => "Control",
            GameMode::Hybrid => "Hybrid",
            GameMode::Payload => "Payload",
            GameMode::Push => "Push",
        }
    }
}

impl TableItemDisplay for GameMode {
    fn render(&self) -> Html {
        html! {
            <td>
                { self.to_string() }
            </td>
        }
    }
}

impl DropdownItemDisplay for GameMode {
    fn render(&self) -> Html {
        html! {
            <div class="item">
                { self.to_string() }
            </div>
        }
    }
}

impl MultiSelectDropdownItemDisplay for GameMode {
    type Layout = ListLayout;

    fn render(&self) -> Html {
        html! {
            <div class="item">
                { self.to_string() }
            </div>
        }
    }
}



impl RainfrogDebug for GameMode {
    fn debug_info(&self) -> String {
        format!("GameMode::{}", self.to_string())
    }
}
