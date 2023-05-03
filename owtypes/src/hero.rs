use rainfrog::debug::RainfrogDebug;
// app/hero.rs
use yew::html;
use yew::Html;

use rainfrog::multi_select_dropdown::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Hero {
    Ashe,
    Bastion,
    Genji,
    Hanzo,
    Junkrat,
    McCree,
    Mei,
    Pharah,
    Reaper,
    Soldier76,
    Sombra,
    Symmetra,
    Torbjorn,
    Tracer,
    Widowmaker,
    Echo,
    Sojourn,
    Ana,
    Baptiste,
    Brigitte,
    Lucio,
    Mercy,
    Moira,
    Zenyatta,
    Kiriko,
    LifeWeaver,
    Reinhardt,
    Orisa,
    Winston,
    WreckingBall,
    DVa,
    Roadhog,
    Sigma,
    Zarya,
    Doomfist,
    Ramattra,
    JunkerQueen,
}

impl Hero {
    pub fn to_string(&self) -> &str {
        match self {
            Hero::Ashe => "Ashe",
            Hero::Bastion => "Bastion",
            Hero::Genji => "Genji",
            Hero::Hanzo => "Hanzo",
            Hero::Junkrat => "Junkrat",
            Hero::McCree => "McCree",
            Hero::Mei => "Mei",
            Hero::Pharah => "Pharah",
            Hero::Reaper => "Reaper",
            Hero::Soldier76 => "Soldier: 76",
            Hero::Sombra => "Sombra",
            Hero::Symmetra => "Symmetra",
            Hero::Torbjorn => "Torbjörn",
            Hero::Tracer => "Tracer",
            Hero::Widowmaker => "Widowmaker",
            Hero::Echo => "Echo",
            Hero::Sojourn => "Sojourn",
            Hero::Ana => "Ana",
            Hero::Baptiste => "Baptiste",
            Hero::Brigitte => "Brigitte",
            Hero::Lucio => "Lúcio",
            Hero::Mercy => "Mercy",
            Hero::Moira => "Moira",
            Hero::Zenyatta => "Zenyatta",
            Hero::Kiriko => "Kiriko",
            Hero::LifeWeaver => "Life Weaver",
            Hero::Reinhardt => "Reinhardt",
            Hero::Orisa => "Orisa",
            Hero::Winston => "Winston",
            Hero::WreckingBall => "Wrecking Ball",
            Hero::DVa => "D.Va",
            Hero::Roadhog => "Roadhog",
            Hero::Sigma => "Sigma",
            Hero::Zarya => "Zarya",
            Hero::Doomfist => "Doomfist",
            Hero::Ramattra => "Ramattra",
            Hero::JunkerQueen => "Junker Queen",
        }
    }

    pub fn load_heroes() -> Vec<Self> {
        return vec![
                Hero::Ashe,
                Hero::Bastion,
                Hero::Genji,
                Hero::Hanzo,
                Hero::Junkrat,
                Hero::McCree,
                Hero::Mei,
                Hero::Pharah,
                Hero::Reaper,
                Hero::Soldier76,
                Hero::Sombra,
                Hero::Symmetra,
                Hero::Torbjorn,
                Hero::Tracer,
                Hero::Widowmaker,
                Hero::Echo,
                Hero::Sojourn,
                Hero::Ana,
                Hero::Baptiste,
                Hero::Brigitte,
                Hero::Lucio,
                Hero::Mercy,
                Hero::Moira,
                Hero::Zenyatta,
                Hero::Kiriko,
                Hero::LifeWeaver,
                Hero::Reinhardt,
                Hero::Orisa,
                Hero::Winston,
                Hero::WreckingBall,
                Hero::DVa,
                Hero::Roadhog,
                Hero::Sigma,
                Hero::Zarya,
                Hero::Doomfist,
                Hero::Ramattra,
                Hero::JunkerQueen,
            ];
    }
}



impl MultiSelectDropdownItemDisplay for Hero {
    type Layout = ListLayout;

    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}


impl RainfrogDebug for Hero {
    fn debug_info(&self) -> String {
        format!("Hero::{}", self.to_string())
    }
}