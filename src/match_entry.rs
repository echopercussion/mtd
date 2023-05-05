use std::rc::Rc;

use log::debug;
use yew::html::Scope;
use yew::prelude::*;
use rainfrog::dropdown::Dropdown;
use rainfrog::multi_select_dropdown::*;

use owtypes::game_result::GameResult;
use owtypes::hero::Hero;
use owtypes::ow_map::OWMap;
use owtypes::account::Account;
use owtypes::game_match::GameMatch;
use owtypes::group::Group;

pub enum Msg {
    SubmitMatch,
    SelectAccount(Rc<Account>),
    SelectGameResult(Rc<GameResult>),
    SelectHeroes(Vec<Rc<Hero>>),
    SelectMap(Rc<OWMap>),
    SelectGroup(Vec<Rc<Account>>),
    ClearSelections,
}


pub struct MatchEntry {
    link: Scope<Self>,
    on_submit: Callback<GameMatch>,
    accounts: Vec<Account>,
    friends: Vec<Account>,

    selected_account: Option<Account>,
    selected_game_result: Option<GameResult>,
    selected_heroes: Option<Vec<Hero>>,
    selected_map: Option<OWMap>,
    selected_group: Option<Group>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MatchEntryProps {
    pub on_submit: Callback<GameMatch>,
    pub accounts: Vec<Account>,
    pub friends: Vec<Account>,
}


impl Component for MatchEntry {
    type Message = Msg;
    type Properties = MatchEntryProps;

    fn create(ctx: &Context<Self>) -> Self {
        MatchEntry {
            link: ctx.link().clone(),
            on_submit: ctx.props().on_submit.clone(),
            accounts: ctx.props().accounts.clone(),
            friends: ctx.props().friends.clone(),

            selected_account: None,
            selected_game_result: None,
            selected_heroes: None,
            selected_map: None,
            selected_group: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitMatch => {
                if let (Some(account), Some(game_result), Some(heroes), Some(map), Some(group)) = (
                    self.selected_account.clone(),
                    self.selected_game_result.clone(),
                    self.selected_heroes.clone(),
                    self.selected_map.clone(),
                    self.selected_group.clone(),
                ) {
                    let new_match = GameMatch::new(
                        account,
                        game_result,
                        heroes,
                        map,
                        group,
                        map.game_mode(), // Use the game_mode() method to get the mode for the selected map
                    );


                    // Clear the selections after submitting the match
                    self.on_submit.emit(new_match);
                    debug!("Match Submitted");
                    self.link.send_message(Msg::ClearSelections);
                    true
                } else {
                    debug!("Match Not Submitted, not enough options selected");

                    false
                }
            }
            Msg::ClearSelections => {
                self.selected_account = None;
                self.selected_game_result = None;
                self.selected_heroes = None;
                self.selected_map = None;
                self.selected_group = None;
                debug!("Selections Cleared");
                true
            }
            Msg::SelectAccount(account) => {
                self.selected_account = match Rc::try_unwrap(account) {
                    Ok(value) => Some(value),
                    Err(_) => {
                        // Handle the error case here, e.g., log an error message.
                        None
                    }
                };
                true
            }
            Msg::SelectGameResult(game_result) => {
                self.selected_game_result = match Rc::try_unwrap(game_result) {
                    Ok(value) => Some(value),
                    Err(_) => {
                        // Handle the error case here, e.g., log an error message.
                        None
                    }
                };
                true
            }
            Msg::SelectHeroes(heroes) => {
                self.selected_heroes = Some(
                    heroes
                        .into_iter()
                        .map(|h| match Rc::try_unwrap(h) {
                            Ok(value) => value,
                            Err(_) => {
                                // Handle the error case here, e.g., log an error message.
                                // Replace Hero::default() with an appropriate default value for your struct.
                                Hero::Tracer
                            }
                        })
                        .collect(),
                );
                true
            }
            Msg::SelectMap(map) => {
                self.selected_map = match Rc::try_unwrap(map) {
                    Ok(value) => Some(value),
                    Err(_) => {
                        // Handle the error case here, e.g., log an error message.
                        None
                    }
                };
                true
            }
            
            Msg::SelectGroup(accounts) => {
                if !accounts.is_empty() {
                    let mut group = Group::new();
                    for account in accounts.into_iter() {
                        let result = group.add_account(Rc::try_unwrap(account).unwrap_or_else(|rc| (*rc).clone()));
                        if let Err(e) = result {
                            // Handle the error case here, e.g., log an error message.
                            log::error!("Failed to add account to group: {:?}", e);
                        }
                    }
                    self.selected_group = Some(group);
                } else {
                    self.selected_group = None;
                }
                true
            }
            
            
            
        }
    }
    

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let game_results = GameResult::load_game_results();
        let rc_game_results: Vec<Rc<GameResult>> = game_results.into_iter().map(Rc::new).collect();

        let heroes = Hero::load_heroes();
        let rc_heroes: Vec<Rc<Hero>> = heroes.into_iter().map(Rc::new).collect();
    
        let maps = OWMap::load_maps();
        let rc_maps: Vec<Rc<OWMap>> = maps.into_iter().map(Rc::new).collect();
    
        let rc_accounts: Vec<Rc<Account>> = self.accounts.iter().cloned().map(Rc::new).collect();
        let rc_friends: Vec<Rc<Account>> = self.friends.iter().cloned().map(Rc::new).collect();
        
        html! {
            <>
                <Dropdown<GameResult>
                    options={rc_game_results}
                    on_select={self.link.callback(Msg::SelectGameResult)}
                />
                <Dropdown<Account>
                    options={rc_accounts.clone()} // Use the Rc version of accounts
                    on_select={self.link.callback(Msg::SelectAccount)}
                />
                <MultiSelectDropdown<Hero>
                    options={rc_heroes}
                    on_select={self.link.callback(Msg::SelectHeroes)}
                />
                <Dropdown<OWMap>
                    options={rc_maps}
                    on_select={self.link.callback(Msg::SelectMap)}
                />
                <MultiSelectDropdown<Account>
                    options={rc_friends}
                    on_select={self.link.callback(Msg::SelectGroup)}
                    max_selections={Some(4)}
                />
                
                <button onclick={self.link.callback(|_| Msg::SubmitMatch)}>
                    {"Submit"}
                </button>
            </>
        }
        
    }
}
