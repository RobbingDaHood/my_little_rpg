use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    my_little_rpg_errors::MyError,
    the_world::{
        attack_types::DamageType,
        treasure_types::{pay_crafting_cost, TreasureType, TreasureType::Gold},
    },
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandElementsReport {
    new_element_type: DamageType,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandElementsReport, MyError> {
    let difficulty_max_resistance_number = game.difficulty.max_resistance.len();
    if difficulty_max_resistance_number >= DamageType::get_all().len() {
        return Err(MyError::create_execute_command_error(
            "Already at maximum elements.".to_string(),
        ));
    }

    //Crafting cost
    let crafting_cost = execute_expand_elements_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Add new element
    let new_element = &DamageType::get_all()[difficulty_max_resistance_number];
    game.difficulty
        .max_resistance
        .insert(new_element.clone(), 2);
    game.difficulty
        .min_resistance
        .insert(new_element.clone(), 1);

    Ok(ExecuteExpandElementsReport {
        new_element_type: new_element.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_elements_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_elements_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.difficulty.max_resistance.len() * 10) as u64)])
}
