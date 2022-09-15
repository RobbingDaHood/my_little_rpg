use serde::{Deserialize, Serialize};

use crate::difficulty::Difficulty;
use crate::item_modifier::Modifier;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Item {
    pub(crate) modifiers: Vec<Modifier>,
    pub(crate) crafting_info: CraftingInfo,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CraftingInfo {
    pub(crate) possible_rolls: Difficulty,
    pub(crate) places_count: usize,
}

#[cfg(test)]
pub mod test_util {
    use crate::Game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::Modifier;

    pub fn create_item(game: &Game) -> Item {
        Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                },
                Modifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                },
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        }
    }
}