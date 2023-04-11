mod tests;

use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::attack_types::AttackType;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMinElementReport {
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

pub fn execute(game: &mut Game) -> Result<ExecuteExpandMinElementReport, MyError> {
    //Crafting cost
    let crafting_cost = execute_expand_min_element_calculate_cost(game);
    let crafting_gold_cost = crafting_cost.get(&Gold).unwrap();

    let max_possible_elements: Vec<&AttackType> = game.difficulty.min_resistance.iter()
        .filter(|(attack_type, amount)| game.difficulty.max_resistance.get(attack_type).unwrap() > &(*amount + crafting_gold_cost))
        .map(|(attack_type, _)| attack_type)
        .collect();

    if max_possible_elements.is_empty() {
        return Err(MyError::create_execute_command_error("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()));
    }

    //Increase min of existing element
    let picked_element = game.random_generator_state.gen_range(0..max_possible_elements.len());
    let picked_element = max_possible_elements[picked_element].clone();

    pay_crafting_cost(game, &crafting_cost)?;

    *game.difficulty.min_resistance.get_mut(&picked_element).unwrap() += crafting_gold_cost;

    Ok(ExecuteExpandMinElementReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_min_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_min_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, game.difficulty.min_resistance.values().sum::<u64>() / game.difficulty.min_resistance.len() as u64)])
}
