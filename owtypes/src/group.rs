use yew::prelude::*;

use rainfrog::table::{TableItemDisplay};
use rainfrog::dropdown::{DropdownItemDisplay};
use rainfrog::multi_select_dropdown::{MultiSelectDropdownItemDisplay, ListLayout};
use rainfrog::debug::RainfrogDebug;

use crate::account::Account;


#[derive(Clone, PartialEq)]
pub struct Group {
    accounts: Vec<Account>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            accounts: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) -> Result<(), &'static str> {
        if self.accounts.len() < 4 {
            self.accounts.push(account);
            Ok(())
        } else {
            Err("Group is already full.")
        }
    }

    pub fn remove_account(&mut self, account: &Account) -> Result<(), &'static str> {
        if let Some(index) = self.accounts.iter().position(|a| a == account) {
            self.accounts.remove(index);
            Ok(())
        } else {
            Err("Account not found in the group.")
        }
    }
}

impl TableItemDisplay for Group {
    fn render(&self) -> Html {
        html! {
            <td>
                { self.accounts.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ") }
            </td>
        }
    }
}

impl DropdownItemDisplay for Group {
    fn render(&self) -> Html {
        html! {
            <div class="item">
                { self.accounts.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ") }
            </div>
        }
    }
}

impl MultiSelectDropdownItemDisplay for Group {
    type Layout = ListLayout;

    fn render(&self) -> Html {
        html! {
            <div class="item">
                { self.accounts.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ") }
            </div>
        }
    }
}


impl RainfrogDebug for Group {
    fn debug_info(&self) -> String {
        let accounts_str = self
            .accounts
            .iter()
            .map(|account| account.debug_info())
            .collect::<Vec<_>>()
            .join(", ");
        format!("Group {{ accounts: [{}] }}", accounts_str)
    }
}