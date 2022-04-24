use std::collections::HashMap;
use rand_pcg::Lcg64Xsh32;
use crate::place::Place;

use serde::{Deserialize, Serialize};
use crate::treasure_types::TreasureType;
use crate::item::Item;
use crate::item_resource::ItemResourceType;
use crate::place_generator::Difficulty;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub(crate) places: Vec<Place>,
    pub(crate) equipped_items: Vec<Item>,
    pub(crate) inventory: Vec<Item>,
    pub(crate) difficulty: Difficulty,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<ItemResourceType, u64>,
    pub(crate) seed: [u8; 16],
    pub(crate) random_generator_state: Lcg64Xsh32,
}