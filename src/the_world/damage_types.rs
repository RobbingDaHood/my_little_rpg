use std::{
    collections::{
        hash_map::{Entry, OccupiedEntry}, HashMap,
    },
    hash::Hash,
};

use rand::prelude::SliceRandom;
use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

use crate::{
    my_little_rpg_errors::MyError,
    the_world::damage_types::DamageType::{
        Corruption, Darkness, Fire, Frost, Holy, Light, Lightning, Nature, Physical,
    },
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

pub fn get_random_attack_type_from_unlocked_new(
    random_generator_state: &mut Lcg64Xsh32,
    unlocks: &HashMap<DamageType, u64>,
) -> Result<(DamageType, u64), MyError> {
    let picked_type = get_random_attack_type(random_generator_state, unlocks, &|_, _| true)?;
    let picked_value_amount = unlocks
        .get(&picked_type)
        .expect("Just fetched this key from the map, so it should be Occupied");
    Ok((picked_type, *picked_value_amount))
}

fn get_random_attack_type(
    random_generator_state: &mut Lcg64Xsh32,
    unlocks: &HashMap<DamageType, u64>,
    condition: &dyn Fn(&DamageType, &u64) -> bool,
) -> Result<DamageType, MyError> {
    let mut attack_values: Vec<&DamageType> = unlocks
        .iter()
        .filter(|(damage_kind, damage_amount)| condition(damage_kind, damage_amount))
        .map(|(damage_kind, _damage_amount)| damage_kind)
        .collect();

    attack_values.sort();
    attack_values
        .choose(random_generator_state)
        .ok_or(MyError::create_execute_command_error(
            "The given Hashmap is empty!".to_string(),
        ))
        .map(|&damage_type| damage_type.clone())
}

pub fn get_mut_random_attack_type<'a>(
    random_generator_state: &mut Lcg64Xsh32,
    unlocks: &'a mut HashMap<DamageType, u64>,
    condition: &dyn Fn(&DamageType, &u64) -> bool,
) -> Result<OccupiedEntry<'a, DamageType, u64>, MyError> {
    let picked_type = get_random_attack_type(random_generator_state, unlocks, condition)?;
    match unlocks.entry(picked_type) {
        Entry::Occupied(entry) => Ok(entry),
        Entry::Vacant(_) => panic!("Just fetched this key from the map, so it should be Occupied"),
    }
}
