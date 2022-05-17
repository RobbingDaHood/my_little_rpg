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