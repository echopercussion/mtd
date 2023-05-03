use yew::Html;
use yew::html;

use rainfrog::dropdown::*;
use rainfrog::table::*;
use rainfrog::debug::RainfrogDebug;


use crate::game_mode::GameMode;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OWMap {
    Dorado,
    Junkertown,
    Havana,
    Rialto,
    Route66,
    WatchpointGibraltar,
    ShambaliMonastary,
    CircuitRoyal,
    BlizzardWorld,
    Eichenwalde,
    Hollywood,
    KingsRow,
    Numbani,
    Midtown,
    Busan,
    Ilios,
    LijiangTower,
    Nepal,
    Oasis,
    AntarcticPeninsula,
    Colosseo,
    Esperança,
    NewQueenStreet,
}

impl OWMap {
    pub fn to_string(&self) -> &str {
        match self {
            OWMap::Dorado => "Dorado",
            OWMap::Junkertown => "Junkertown",
            OWMap::Havana => "Havana",
            OWMap::Rialto => "Rialto",
            OWMap::Route66 => "Route 66",
            OWMap::WatchpointGibraltar => "Watchpoint: Gibraltar",
            OWMap::ShambaliMonastary => "Shambali Monastary",
            OWMap::CircuitRoyal => "Circuit Royal",
            OWMap::BlizzardWorld => "Blizzard World",
            OWMap::Eichenwalde => "Eichenwalde",
            OWMap::Hollywood => "Hollywood",
            OWMap::KingsRow => "King's Row",
            OWMap::Numbani => "Numbani",
            OWMap::Midtown => "Midtown",
            OWMap::Busan => "Busan",
            OWMap::Ilios => "Ilios",
            OWMap::LijiangTower => "Lijiang Tower",
            OWMap::Nepal => "Nepal",
            OWMap::Oasis => "Oasis",
            OWMap::AntarcticPeninsula => "Antarctic Peninsula",
            OWMap::Colosseo => "Colosseo",
            OWMap::Esperança => "Esperança",
            OWMap::NewQueenStreet => "New Queen Street",
        }
    }

    pub fn load_maps() -> Vec<OWMap> {
        return vec![            
            OWMap::Dorado,
            OWMap::Junkertown,
            OWMap::Havana,
            OWMap::Rialto,
            OWMap::Route66,
            OWMap::WatchpointGibraltar,
            OWMap::ShambaliMonastary,
            OWMap::CircuitRoyal,
            OWMap::BlizzardWorld,
            OWMap::Eichenwalde,
            OWMap::Hollywood,
            OWMap::KingsRow,
            OWMap::Numbani,
            OWMap::Midtown,
            OWMap::Busan,
            OWMap::Ilios,
            OWMap::LijiangTower,
            OWMap::Nepal,
            OWMap::Oasis,
            OWMap::AntarcticPeninsula,
            OWMap::Colosseo,
            OWMap::Esperança,
            OWMap::NewQueenStreet,
        ]
    }

    pub fn game_mode(&self) -> GameMode {
        match self {
            OWMap::Dorado
            | OWMap::Junkertown
            | OWMap::Havana
            | OWMap::Rialto
            | OWMap::Route66
            | OWMap::WatchpointGibraltar
            | OWMap::ShambaliMonastary
            | OWMap::CircuitRoyal => GameMode::Payload,

            OWMap::BlizzardWorld
            | OWMap::Eichenwalde
            | OWMap::Hollywood
            | OWMap::KingsRow
            | OWMap::Numbani
            | OWMap::Midtown => GameMode::Hybrid,

            OWMap::Busan
            | OWMap::Ilios
            | OWMap::LijiangTower
            | OWMap::Nepal
            | OWMap::Oasis
            | OWMap::AntarcticPeninsula => GameMode::Control,

            OWMap::Colosseo 
            | OWMap::Esperança 
            | OWMap::NewQueenStreet => GameMode::Push,
        }
    }
}


impl DropdownItemDisplay for OWMap {
    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}

impl TableItemDisplay for OWMap {
    fn render(&self) -> Html {
        html! {
            <span>{ self.to_string() }</span>
        }
    }
}

impl RainfrogDebug for OWMap {
    fn debug_info(&self) -> String {
        format!("OWMap::{}", self.to_string())
    }
}