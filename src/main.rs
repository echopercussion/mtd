// main.rs
use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::html::*;

use owtypes::account::Account;
use owtypes::game_match::GameMatch;

use rainfrog::rainfrog_logger::RainfrogLogger;

use mtd::match_entry::MatchEntry;



fn main() {
    // Initialize the logger
    run_app();
}


pub enum Msg {
    AddMatch(GameMatch),
}

pub struct App {
    link: Scope<Self>,
    local_matches: Vec<GameMatch>,
    accounts: Vec<Account>,
    friends: Vec<Account>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            local_matches: Vec::new(),
            accounts: load_test_accounts(), // You should replace this with your actual implementation
            friends: load_test_friends(),   // You should replace this with your actual implementation
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddMatch(new_match) => {
                self.local_matches.push(new_match);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                </header>
                { self.render_body(ctx) }
            </>
        }
    }
}

impl App {
    fn render_body(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MatchEntry
                on_submit={ctx.link().callback(Msg::AddMatch)}
                accounts={self.accounts.clone()}
                friends={self.friends.clone()}
            />
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app() {

    RainfrogLogger::init().expect("Failed to initialize RainfrogLogger");

    yew::Renderer::<App>::new().render();
}







// Test functionality to be replaced in the future
fn load_test_accounts() -> Vec<Account> {
    vec![
        Account::new("Rayla".to_string(), 1337),
        Account::new("Bear".to_string(),42069)
    ]
}

fn load_test_friends() -> Vec<Account>  {
    vec![
        Account::new("Rayla".to_string(), 1337),
        Account::new("Bear".to_string(), 42069),
        Account::new("PlebFan420".to_string(), 23490),
        Account::new("Milftoes38".to_string(), 74532),
        Account::new("tony".to_string(), 5),
    ]
}
