use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::attack_types::AttackType::{Corruption, Darkness, Fire, Frost, Holy, Light, Lightning, Nature, Physical};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
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

    pub fn order_set(collection: &HashSet<&AttackType>) -> Vec<AttackType> {
        AttackType::get_all().iter()
            .filter(|attack_type| collection.contains(attack_type))
            .cloned()
            .collect::<Vec<AttackType>>()
    }

    pub fn order_map(collection: &HashMap<AttackType, u64>) -> Vec<(AttackType, u64)> {
        AttackType::get_all().iter()
            .filter(|attack_type| collection.contains_key(attack_type))
            .map(|attack_type| (attack_type.clone(), *collection.get(attack_type).unwrap()))
            .collect::<Vec<(AttackType, u64)>>()
    }
}