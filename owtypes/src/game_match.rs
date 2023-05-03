use crate::account::Account;
use crate::ow_map::OWMap;
use crate::hero::Hero;
use crate::game_result::GameResult;
use crate::game_mode::GameMode;
use crate::group::Group;

use rainfrog::debug::RainfrogDebug;
use rainfrog::table::*;


#[derive(PartialEq, Clone)]
pub struct GameMatch {
    account: Account,
    map: OWMap,
    mode: GameMode,
    heroes: Vec<Hero>,
    group: Group, 
    game_result: GameResult,
}

impl GameMatch {
    pub fn new(account: Account, game_result: GameResult, heroes: Vec<Hero>, map: OWMap, group: Group, mode: GameMode) -> Self {
        GameMatch {
            account,
            game_result,
            heroes,
            map,
            group,
            mode,
        }
    }
}



impl TableBlocks for GameMatch {
    fn headers() -> Vec<String> {
        vec![
            "Account".to_string(),
            "Map".to_string(),
            "Heroes Played".to_string(),
            "Group".to_string(),
            "Game Result".to_string(),
        ]
    }

    fn fields(&self) -> Vec<Box<dyn TableItemDisplay>> {
        vec![
            // ...
            Box::new(self.map),
            // ...
        ]
    }
}



impl RainfrogDebug for GameMatch {
    fn debug_info(&self) -> String {
        let heroes_str = self.heroes
            .iter()
            .map(|hero| hero.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "GameMatch {{ account: {}, map: {}, mode: {}, heroes: [{}], group: {}, game_result: {} }}",
            self.account.debug_info(),
            self.map.to_string(),
            self.mode.to_string(),
            heroes_str,
            self.group.debug_info(),
            self.game_result.to_string()
        )
    }
}