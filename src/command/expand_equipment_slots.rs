use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    my_little_rpg_errors::MyError,
    the_world::{
        item::Item,
        treasure_types::{pay_crafting_cost, TreasureType, TreasureType::Gold},
    },
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandEquipmentSlotsReport {
    new_equipped_items: Vec<Item>,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_equipment_slots_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandEquipmentSlotsReport, MyError> {
    let Some(first_item_index) = game.inventory.iter().position(Option::is_some) else {
            return Err(MyError::create_execute_command_error(
                "No item in inventory to equip in new item slot. Whole inventory is empty."
                    .to_string(),
            ))
    };

    //Crafting cost
    let crafting_cost = execute_expand_equipment_slots_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    let item = game.inventory[first_item_index].take().unwrap_or_else(|| panic!("Item at index {first_item_index} did exist earlier but does not anymore."));
    game.equipped_items.push(item);

    Ok(ExecuteExpandEquipmentSlotsReport {
        new_equipped_items: game.equipped_items.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_equipment_slots_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_equipment_slots_calculate_cost(game: &Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.equipped_items.len() + 1).pow(5) as u64)])
}
