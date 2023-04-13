use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use rand::prelude::SliceRandom;
use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

use crate::the_world::attack_types::AttackType::{Corruption, Darkness, Fire, Frost, Holy, Light, Lightning, Nature, Physical};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, PartialOrd, Ord)]
pub enum AttackType {
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

impl AttackType {
    //TODO replace all get_all with static fields; or at least them return that.
    pub fn get_all() -> Vec<AttackType> {
        vec![
            Physical,
            Fire,
            Frost,
            Lightning,
            Light,
            Darkness,
            Nature,
            Corruption,
            Holy,
        ]
    }
}

pub fn get_random_attack_type_from_unlocked(random_generator_state: &mut Lcg64Xsh32, unlocks: &HashMap<AttackType, u64>) -> AttackType {
    let mut attack_values: Vec<&AttackType> = unlocks.keys().collect();
    attack_values.sort();
    attack_values.choose(random_generator_state).unwrap().deref().clone()
}
