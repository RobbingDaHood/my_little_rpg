use std::collections::HashMap;

use rand::prelude::SliceRandom;
use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

use crate::the_world::attack_types::AttackType;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::game_statistics::GameStatistics;
use crate::the_world::item::Item;
use crate::the_world::item_resource::Type;
use crate::the_world::place::Place;
use crate::the_world::treasure_types::TreasureType;

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
    pub(crate) game_statistics: GameStatistics,
}

pub fn get_random_attack_type_from_unlocked(random_generator_state: &mut Lcg64Xsh32, unlocks: &HashMap<AttackType, u64>) -> AttackType {
    let attack_type = AttackType::get_all().into_iter()
        .filter(|attack_type| unlocks.contains_key(attack_type))
        .collect::<Vec<AttackType>>()
        .choose(random_generator_state)
        .unwrap()
        .clone();
    attack_type
}