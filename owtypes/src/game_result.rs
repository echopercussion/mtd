use std::fmt;

use rainfrog::debug::RainfrogDebug;
use yew::Html;
use yew::html;

use rainfrog::dropdown::*;
use rainfrog::table::*;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    pub fn to_string(&self) -> &str {
        match self {
            GameResult::Win => "Win",
            GameResult::Loss => "Loss",
            GameResult::Draw => "Draw",
        }
    }

    pub fn load_game_results() -> Vec<GameResult> {
        vec![GameResult::Win, GameResult::Loss, GameResult::Draw]
    }
}

// Implement DropdownItemDisplay for GameResult
impl DropdownItemDisplay for GameResult {
    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}

// Implement TableItemDisplay for GameResult
impl TableItemDisplay for GameResult {
    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}

//debug display func
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RainfrogDebug for GameResult {
    fn debug_info(&self) -> String {
        format!("GameResult::{}", self.to_string())
    }
}