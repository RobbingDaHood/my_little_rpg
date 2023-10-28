use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    my_little_rpg_errors::MyError,
    the_world::{
        damage_types::{get_mut_random_attack_type, DamageType},
        difficulty::Difficulty,
        treasure_types::{pay_crafting_cost, TreasureType, TreasureType::Gold},
    },
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMinElementReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_min_element_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandMinElementReport, MyError> {
    //Crafting cost
    let crafting_cost = execute_expand_min_element_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    let min_resistance_diff: u64 = crafting_cost
        .values()
        .fold(0u64, |r, s| r.checked_add(*s).unwrap_or(u64::MAX));

    //Increase max of existing element
    *get_mut_random_attack_type(
        &mut game.random_generator_state,
        &mut game.difficulty.min_resistance,
        &|attack_type, amount| {
            let max_resistance_amount =
                game.difficulty.max_resistance.get(&attack_type).expect(
                    "We expect Max resistance to have the same elements as Min resistance.",
                );
            let possible_new_min_resistance_amount = *amount + min_resistance_diff;
            *max_resistance_amount > possible_new_min_resistance_amount
        },
    )
    .map_err(|e| {
        MyError::create_execute_command_error(
            "There are no element minimum values that can be upgraded, consider expanding a max \
             element value."
                .to_string(),
        )
    })?
    .get_mut() += min_resistance_diff;

    Ok(ExecuteExpandMinElementReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_min_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_min_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(
        Gold,
        game.difficulty.min_resistance.values().sum::<u64>()
            / game.difficulty.min_resistance.len() as u64,
    )])
}
