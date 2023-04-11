mod tests;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::attack_types::get_random_attack_type_from_unlocked;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMaxElementReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result)
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandMaxElementReport, MyError> {
    //Crafting cost
    let crafting_cost = execute_expand_max_element_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Increase max of existing element
    let picked_element = get_random_attack_type_from_unlocked(&mut game.random_generator_state, &game.difficulty.min_resistance);

    *game.difficulty.max_resistance.get_mut(&picked_element).unwrap() += crafting_cost.get(&Gold).unwrap();

    Ok(ExecuteExpandMaxElementReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_max_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_max_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, game.difficulty.max_resistance.values().sum::<u64>() / game.difficulty.max_resistance.len() as u64)])
}
