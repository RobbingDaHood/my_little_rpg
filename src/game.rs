use std::collections::HashMap;
use rand_pcg::Lcg64Xsh32;
use crate::place::Place;

use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::treasure_types::TreasureType;
use crate::item::Item;
use crate::item_resource::ItemResourceType;
use rand::prelude::SliceRandom;
use crate::difficulty::Difficulty;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub(crate) places: Vec<Place>,
    pub(crate) equipped_items: Vec<Item>,
    pub(crate) inventory: Vec<Option<Item>>,
    pub(crate) difficulty: Difficulty,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<ItemResourceType, u64>,
    pub(crate) seed: [u8; 16],
    pub(crate) random_generator_state: Lcg64Xsh32,
}

pub fn get_random_attack_type_from_unlocked(game: &mut Game, given_unlocks: &Option<&Difficulty>) -> AttackType {
    let attack_type = AttackType::get_all().into_iter()
        .filter(|attack_type| given_unlocks.unwrap_or(&game.difficulty).min_resistance.contains_key(attack_type))
        .collect::<Vec<AttackType>>()
        .choose(&mut game.random_generator_state)
        .unwrap()
        .clone();
    attack_type
}