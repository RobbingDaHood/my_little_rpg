use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::command::roll_modifier::execute_craft;
use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::index_specifier::{calculate_absolute_item_indexes, ErrorConditions, IndexSpecifier};
use crate::the_world::item::Item;
use crate::the_world::treasure_types::TreasureType;

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandModifiersReport {
    new_item: Item,
    paid_cost: usize,
    new_cost: usize,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_craft_expand_modifiers_json(game: &mut Game, inventory_index: usize, sacrifice_item_indexes: Vec<IndexSpecifier>) -> Value {
    match execute_craft_expand_modifiers(game, inventory_index, sacrifice_item_indexes) {
        Ok(result) => json!(result),
        Err(result) => json!(result)
    }
}

pub fn execute_craft_expand_modifiers(game: &mut Game, inventory_index: usize, mut sacrifice_item_indexes: Vec<IndexSpecifier>) -> Result<ExecuteExpandModifiersReport, MyError> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(MyError::create_execute_command_error(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len())));
    }
    let inventory_item = game.inventory[inventory_index].as_ref()
        .ok_or_else(|| MyError::create_execute_command_error(format!("inventory_index {} is empty.", inventory_index)))?;
    if usize::from(inventory_item.crafting_info.possible_rolls.min_simultaneous_resistances) <= inventory_item.modifiers.len() {
        return Err(MyError::create_execute_command_error(format!("inventory_index.possible_rolls.min_simultaneous_resistances {} need to be bigger than inventory_index current number of modifiers {} for it to be expanded.",
                                                                 inventory_item.crafting_info.possible_rolls.min_simultaneous_resistances,
                                                                 inventory_item.modifiers.len()))
        );
    }

    let cost = execute_craft_expand_modifiers_calculate_cost(&game, inventory_index);
    if sacrifice_item_indexes.len() < cost {
        return Err(MyError::create_execute_command_error(format!("craft_reroll_modifier needs {} items to be sacrificed but you only provided {}", cost, sacrifice_item_indexes.len())));
    }

    //Only need to cost amount of items
    sacrifice_item_indexes.truncate(cost);

    let error_conditions = get_index_specifier_error_conditions(&inventory_item);
    let calculated_sacrifice_item_indexes = calculate_absolute_item_indexes(&game, inventory_index, &sacrifice_item_indexes, &error_conditions)?;

    //Crafting cost
    for sacrifice_item_index in calculated_sacrifice_item_indexes {
        game.inventory[sacrifice_item_index] = None;
    }

    //Create item
    let new_item_modifier = execute_craft(&mut game.random_generator_state, &game.inventory[inventory_index].as_ref().unwrap().crafting_info);
    game.inventory[inventory_index].as_mut().unwrap().modifiers.push(new_item_modifier);

    Ok(ExecuteExpandModifiersReport {
        //TODO replace all unwrap and expect with better error handling
        new_item: game.inventory[inventory_index].clone().unwrap(),
        paid_cost: cost,
        new_cost: execute_craft_expand_modifiers_calculate_cost(game, inventory_index),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

fn get_index_specifier_error_conditions(inventory_item: &Item) -> ErrorConditions {
    let inventory_item_cloned = inventory_item.clone();
    let enough_modifiers_condition = move |sacrifice_item_index: usize, sacrificed_item: &Item| {
        let crafting_item_modifiers_count = inventory_item_cloned.modifiers.len();
        let sacrificed_item_modifiers_count = sacrificed_item.modifiers.len();
        if sacrificed_item_modifiers_count < crafting_item_modifiers_count {
            Some(MyError::create_execute_command_error(format!("sacrifice_item_index {} need to have at least {} modifiers but it only had {}", sacrifice_item_index, crafting_item_modifiers_count, sacrificed_item_modifiers_count)))
        } else {
            None
        }
    };
    ErrorConditions { error_conditions: vec![Box::new(enough_modifiers_condition)] }
}

pub fn execute_craft_expand_modifiers_calculate_cost(game: &Game, inventory_index: usize) -> usize {
    match &game.inventory[inventory_index] {
        Some(item) => item.modifiers.len() * 2,
        None => 0
    }
}
