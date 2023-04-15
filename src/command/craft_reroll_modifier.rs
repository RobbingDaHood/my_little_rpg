use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    command::roll_modifier::execute_craft,
    my_little_rpg_errors::MyError,
    the_world::{
        index_specifier::{calculate_absolute_item_indexes, ErrorConditions, IndexSpecifier},
        item::Item,
    },
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteCraftRerollModifierReport {
    new_item: Item,
    paid_cost: u16,
    new_cost: u16,
}

pub fn execute_json(
    game: &mut Game,
    inventory_index: usize,
    modifier_index: usize,
    sacrifice_item_indexes: Vec<IndexSpecifier>,
) -> Value {
    match execute(
        game,
        inventory_index,
        modifier_index,
        sacrifice_item_indexes,
    ) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(
    game: &mut Game,
    inventory_index: usize,
    modifier_index: usize,
    mut sacrifice_item_indexes: Vec<IndexSpecifier>,
) -> Result<ExecuteCraftRerollModifierReport, MyError> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(MyError::create_execute_command_error(format!(
            "inventory_index {} is not within the range of the inventory {}",
            inventory_index,
            game.inventory.len()
        )));
    }
    if game.inventory[inventory_index].is_none() {
        return Err(MyError::create_execute_command_error(format!(
            "inventory_index {inventory_index} is empty."
        )));
    }
    let inventory_item = game.inventory[inventory_index].as_ref().ok_or_else(|| {
        MyError::create_execute_command_error(format!(
            "inventory_index {} is empty.",
            inventory_index
        ))
    })?;
    if inventory_item.modifiers.len() <= modifier_index {
        return Err(MyError::create_execute_command_error(format!(
            "modifier_index {} is not within the range of the item modifiers {}",
            modifier_index.clone(),
            inventory_item.modifiers.len()
        )));
    }

    //Crafting cost
    let cost = execute_craft_reroll_modifier_calculate_cost(game, inventory_index);
    if sacrifice_item_indexes.len() < cost.into() {
        return Err(MyError::create_execute_command_error(format!(
            "craft_reroll_modifier needs {} items to be sacrificed but you only provided {}",
            cost,
            sacrifice_item_indexes.len()
        )));
    }

    //Only need to sacrifice cost amount of items
    sacrifice_item_indexes.truncate(usize::from(cost));

    let error_conditions = get_index_specifier_error_conditions(modifier_index);
    let calculated_sacrifice_item_indexes = calculate_absolute_item_indexes(
        game,
        inventory_index,
        &sacrifice_item_indexes,
        &error_conditions,
    )?;

    //Create item
    let new_item_modifier = execute_craft(
        &mut game.random_generator_state,
        &inventory_item.crafting_info,
    );

    //Crafting cost
    for sacrifice_item_index in &calculated_sacrifice_item_indexes {
        game.inventory[*sacrifice_item_index] = None;
    }

    let mut inventory_item = game.inventory[inventory_index].as_mut().expect(
        format!(
            "Item at index {} did exist earlier but does not anymore.",
            inventory_index
        )
        .as_str(),
    );
    inventory_item.modifiers[modifier_index] = new_item_modifier;

    Ok(ExecuteCraftRerollModifierReport {
        new_item: inventory_item.clone(),
        new_cost: execute_craft_reroll_modifier_calculate_cost(game, inventory_index),
        paid_cost: cost,
    })
}

fn get_index_specifier_error_conditions(modifier_index: usize) -> ErrorConditions {
    let enough_modifiers_condition = move |sacrifice_item_index: usize, sacrificed_item: &Item| {
        if sacrificed_item.modifiers.len() <= modifier_index {
            Some(MyError::create_execute_command_error(format!(
                "sacrifice_item_index {} need to have at least {} modifiers but it only had {}",
                sacrifice_item_index,
                modifier_index + 1,
                sacrificed_item.modifiers.len()
            )))
        } else {
            None
        }
    };
    vec![Box::new(enough_modifiers_condition)]
}

pub fn execute_craft_reroll_modifier_calculate_cost(
    game: &Game,
    inventory_index: usize,
) -> u16 {
    match &game.inventory[inventory_index] {
        Some(item) => u16::try_from(item.modifiers.len()).unwrap_or(u16::MAX),
        None => 0,
    }
}
