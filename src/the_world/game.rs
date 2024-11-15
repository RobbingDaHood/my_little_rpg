use std::collections::HashMap;

use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

use crate::the_world::{
    difficulty::Difficulty, game_statistics::GameStatistics, item::Item, item_resource::Type,
    place::Place, treasure_types::TreasureType,
};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub(crate) places: Vec<Place>,
    pub(crate) equipped_items: Vec<Item>,
    pub(crate) inventory: Vec<Option<Item>>,
    pub(crate) difficulty: Difficulty,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<Type, u64>,
    pub(crate) seed: [u8; 16],
    pub(crate) random_generator_state: Lcg64Xsh32,
    pub(crate) statistics: GameStatistics,
}
