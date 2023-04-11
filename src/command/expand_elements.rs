mod tests;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::attack_types::AttackType;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandElementsReport {
    new_element_type: AttackType,
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

pub fn execute(game: &mut Game) -> Result<ExecuteExpandElementsReport, MyError> {
    if game.difficulty.max_resistance.len() >= AttackType::get_all().len() {
        return Err(MyError::create_execute_command_error("Already at maximum elements.".to_string()));
    }

    //Crafting cost
    let crafting_cost = execute_expand_elements_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Add new element
    let new_element = AttackType::get_all()[game.difficulty.max_resistance.len()].clone();
    game.difficulty.max_resistance.insert(new_element.clone(), 2);
    game.difficulty.min_resistance.insert(new_element.clone(), 1);

    Ok(ExecuteExpandElementsReport {
        new_element_type: new_element,
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_elements_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_elements_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.difficulty.max_resistance.len() * 10) as u64)])
}
