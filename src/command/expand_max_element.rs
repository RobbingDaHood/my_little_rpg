use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    my_little_rpg_errors::MyError,
    the_world::{
        damage_types::get_mut_random_attack_type,
        difficulty::Difficulty,
        treasure_types::{pay_crafting_cost, TreasureType, TreasureType::Gold},
    },
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMaxElementReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_max_element_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandMaxElementReport, MyError> {
    //Crafting cost
    let crafting_cost = execute_expand_max_element_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    let max_resistance_diff: u64 = crafting_cost
        .values()
        .fold(0u64, |r, s| r.checked_add(*s).unwrap_or(u64::MAX));

    //Increase max of existing element
    *get_mut_random_attack_type(
        &mut game.random_generator_state,
        &mut game.difficulty.max_resistance,
        &|_,_| true
    )?
    .get_mut() += max_resistance_diff;

    Ok(ExecuteExpandMaxElementReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_max_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_max_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(
        Gold,
        game.difficulty.max_resistance.values().sum::<u64>()
            / game.difficulty.max_resistance.len() as u64,
    )])
}
