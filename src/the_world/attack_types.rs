use std::{collections::HashMap, ops::Deref};

use rand::prelude::SliceRandom;
use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

use crate::the_world::attack_types::DamageType::{
    Corruption, Darkness, Fire, Frost, Holy, Light, Lightning, Nature, Physical,
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, PartialOrd, Ord)]
pub enum DamageType {
    Physical,
    Fire,
    Frost,
    Lightning,
    Light,
    Darkness,
    Nature,
    Corruption,
    Holy,
}

//TODO: Could also create a resource if more than half damage goes through; Then that resource could be used in crafting and modifiers.

impl DamageType {
    //TODO replace all get_all with static fields; or at least them return that.
    pub fn get_all() -> Vec<DamageType> {
        vec![
            Physical, Fire, Frost, Lightning, Light, Darkness, Nature, Corruption, Holy,
        ]
    }
}

pub fn get_random_attack_type_from_unlocked(
    random_generator_state: &mut Lcg64Xsh32,
    unlocks: &HashMap<DamageType, u64>,
) -> DamageType {
    let mut attack_values: Vec<&DamageType> = unlocks.keys().collect();
    attack_values.sort();
    attack_values
        .choose(random_generator_state)
        .unwrap()
        .deref()
        .clone()
}
