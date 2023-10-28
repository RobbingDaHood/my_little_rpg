use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    my_little_rpg_errors::MyError,
    the_world::treasure_types::{pay_crafting_cost, TreasureType, TreasureType::Gold},
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMaxSimultaneousElementReport {
    new_max_simultaneous_resistances: u8,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_max_simultaneous_element_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandMaxSimultaneousElementReport, MyError> {
    if (game.difficulty.max_simultaneous_resistances as usize)
        >= game.difficulty.max_resistance.len()
    {
        return Err(MyError::create_execute_command_error(format!(
            "max_simultaneous_resistances {} is already equal to number of active resistances {}. \
             Consider calling ExpandElements.",
            game.difficulty.max_simultaneous_resistances,
            game.difficulty.max_resistance.len()
        )));
    }

    //Crafting cost
    let crafting_cost = execute_expand_max_simultaneous_element_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Increase max of existing element
    game.difficulty.max_simultaneous_resistances += 1;

    Ok(ExecuteExpandMaxSimultaneousElementReport {
        new_max_simultaneous_resistances: game.difficulty.max_simultaneous_resistances,
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_max_simultaneous_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_max_simultaneous_element_calculate_cost(
    game: &mut Game
) -> HashMap<TreasureType, u64> {
    HashMap::from([(
        Gold,
        u64::from(game.difficulty.max_simultaneous_resistances) * 10,
    )])
}
