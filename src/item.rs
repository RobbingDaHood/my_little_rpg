use crate::item_modifier::ItemModifier;
use serde::{Deserialize, Serialize};
use crate::difficulty::Difficulty;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Item {
    pub(crate) modifiers: Vec<ItemModifier>,
    pub(crate) crafting_info: CraftingInfo,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CraftingInfo {
    pub(crate) possible_rolls: Difficulty,
    pub(crate) places_count: usize,
}

#[cfg(test)]
pub mod test_util {
    use crate::Game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::ItemModifier;

    pub fn create_item(game: &Game) -> Option<Item> {
        Some(Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                },
                ItemModifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                },
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        })
    }
}