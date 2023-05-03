use rainfrog::debug::RainfrogDebug;
use yew::Html;
use yew::html;

use rainfrog::dropdown::*;
use rainfrog::table::*;
use rainfrog::multi_select_dropdown::*;
use rainfrog::string_display::StringDisplay;



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Account {
    username: String,
    bnet_id: usize,
}

impl Account {
    // Create a new Account instance
    pub fn new(username: String, bnet_id: usize) -> Self {
        Account {
            username,
            bnet_id,
        }
    }
    // Implement to_string method
    pub fn to_string(&self) -> String {
        format!("{}#{}", self.username, self.bnet_id)
    }
}

// Implement DropdownItemDisplay for Account
impl DropdownItemDisplay for Account {
    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}

// Implement TableItemDisplay for Account
impl TableItemDisplay for Account {
    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}

// Implement TableBlocks for Account
impl TableBlocks for Account {
    fn headers() -> Vec<String> {
        vec!["Username".to_string(), "BNet ID".to_string()]
    }

    fn fields(&self) -> Vec<Box<dyn TableItemDisplay>> {
        vec![
            Box::new(StringDisplay::new(self.username.clone())),
            Box::new(StringDisplay::new(self.bnet_id.to_string())),
        ]
    }
}

impl MultiSelectDropdownItemDisplay for Account {
    type Layout = ListLayout;

    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}


impl RainfrogDebug for Account {
    fn debug_info(&self) -> String {
        format!("Account {{ username: {}, bnet_id: {} }}", self.username, self.bnet_id)
    }
}