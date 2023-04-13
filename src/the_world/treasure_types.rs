use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::my_little_rpg_errors::MyError;
use crate::the_world::treasure_types::TreasureType::Gold;
use crate::Game;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum TreasureType {
    Gold,
}

impl TreasureType {
    pub fn get_all() -> Vec<TreasureType> {
        vec![Gold]
    }
}

//TODO consider moving code and add some tests
pub fn pay_crafting_cost(
    game: &mut Game,
    crafting_cost: &HashMap<TreasureType, u64>,
) -> Result<(), MyError> {
    if calculate_are_all_treasure_payable(&game.treasure, crafting_cost) {
        update_all_treasure(&mut game.treasure, crafting_cost);
    } else {
        return Err(MyError::create_execute_command_error(format!(
            "Cant pay the crafting cost, the cost is {:?} and you only have {:?}",
            crafting_cost, game.treasure
        )));
    }
    Ok(())
}

fn calculate_are_all_treasure_payable(
    current_treasure: &HashMap<TreasureType, u64>,
    treasure_cost: &HashMap<TreasureType, u64>,
) -> bool {
    treasure_cost.iter().all(|(item_resource_type, amount)| {
        match current_treasure.get(item_resource_type) {
            None => false,
            Some(stored_amount) => stored_amount >= amount,
        }
    })
}

fn update_all_treasure(
    current_treasure: &mut HashMap<TreasureType, u64>,
    treasure_cost: &HashMap<TreasureType, u64>,
) {
    for (treasure_type, amount) in treasure_cost {
        *current_treasure.entry(treasure_type.clone()).or_insert(0) -= amount;
    }
}
