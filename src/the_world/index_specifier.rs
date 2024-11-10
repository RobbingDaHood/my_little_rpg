use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{my_little_rpg_errors::MyError, the_world::item::Item, Game};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum IndexSpecifier {
    Absolute(usize),
    RelativePositive(usize),
    RelativeNegative(usize),
}

type ErrorCondition = dyn Fn(usize, &Item) -> Option<MyError>;
pub(crate) type ErrorConditions = Vec<Box<ErrorCondition>>;

//TODO moving this
pub fn calculate_absolute_item_indexes(
    game: &Game,
    inventory_index: usize,
    index_specifiers: &[IndexSpecifier],
    error_conditions: &ErrorConditions,
) -> Result<HashSet<usize>, MyError> {
    let mut calculated_selected_item_indexes = HashSet::new();
    for index_specifier in index_specifiers {
        let valid_index = get_index(
            game,
            inventory_index,
            &calculated_selected_item_indexes,
            index_specifier,
            error_conditions,
        )?;
        calculated_selected_item_indexes.insert(valid_index);
    }
    Ok(calculated_selected_item_indexes)
}

fn get_index(
    game: &Game,
    inventory_index: usize,
    calculated_selected_item_indexes: &HashSet<usize>,
    index_specifier: &IndexSpecifier,
    error_conditions: &ErrorConditions,
) -> Result<usize, MyError> {
    match index_specifier {
        IndexSpecifier::Absolute(index) => {
            get_absolute_index(
                game,
                inventory_index,
                calculated_selected_item_indexes,
                index_specifier,
                *index,
                error_conditions,
            )
        }
        IndexSpecifier::RelativePositive(relative_index) => {
            get_relative_positive_index(
                game,
                inventory_index,
                calculated_selected_item_indexes,
                index_specifier,
                *relative_index,
                error_conditions,
            )
        }
        IndexSpecifier::RelativeNegative(relative_index) => {
            get_relative_negative_index(
                game,
                inventory_index,
                calculated_selected_item_indexes,
                index_specifier,
                *relative_index,
                error_conditions,
            )
        }
    }
}

fn get_absolute_index(
    game: &Game,
    inventory_index: usize,
    calculated_selected_item_indexes: &HashSet<usize>,
    index_specifier: &IndexSpecifier,
    candidate_index: usize,
    error_conditions: &ErrorConditions,
) -> Result<usize, MyError> {
    if inventory_index == candidate_index {
        return Err(MyError::create_execute_command_error(format!(
            "inventory_index {inventory_index} and index_specifier {index_specifier:?} cannot be \
             the same"
        )));
    }
    if game.inventory.len() <= candidate_index {
        return Err(MyError::create_execute_command_error(format!(
            "index_specifier {:?} is not within the range of the inventory {}",
            index_specifier,
            game.inventory.len()
        )));
    }
    let candidate_item = game.inventory[candidate_index].as_ref().ok_or_else(|| {
        MyError::create_execute_command_error(format!(
            "index_specifier {index_specifier:?} is pointing at empty inventory slot."
        ))
    })?;
    if calculated_selected_item_indexes.contains(&candidate_index) {
        return Err(MyError::create_execute_command_error(format!(
            "index_specifier {index_specifier:?} is already present in calculated sacrifice \
             indexes {calculated_selected_item_indexes:?}"
        )));
    };
    handle_conditions(candidate_index, error_conditions, candidate_item)?;
    Ok(candidate_index)
}

fn handle_conditions(
    candidate_index: usize,
    error_conditions: &ErrorConditions,
    candidate_item: &Item,
) -> Result<usize, MyError> {
    let possible_error = error_conditions
        .iter()
        .find_map(|condition| condition(candidate_index, candidate_item));
    match possible_error {
        Some(error) => Err(error),
        None => Ok(candidate_index),
    }
}

fn get_relative_positive_index(
    game: &Game,
    inventory_index: usize,
    calculated_selected_item_indexes: &HashSet<usize>,
    index_specifier: &IndexSpecifier,
    relative_index: usize,
    error_conditions: &ErrorConditions,
) -> Result<usize, MyError> {
    let start_index = inventory_index + relative_index;
    game.inventory
        .iter()
        .enumerate()
        .skip(start_index)
        .filter_map(|(index, item)| item.as_ref().map(|unwrapped_item| (index, unwrapped_item)))
        .find(|(i, _)| !calculated_selected_item_indexes.contains(i))
        .map(|(index, item)| handle_conditions(index, error_conditions, item))
        .ok_or_else(|| {
            MyError::create_execute_command_error(format!(
                "index_specifier: {index_specifier:?} did not find any items in inventory from relative point {start_index} \
                 until end of inventory."
            ))
        })?
}

fn get_relative_negative_index(
    game: &Game,
    inventory_index: usize,
    calculated_selected_item_indexes: &HashSet<usize>,
    index_specifier: &IndexSpecifier,
    relative_index: usize,
    error_conditions: &ErrorConditions,
) -> Result<usize, MyError> {
    let start_index = inventory_index - relative_index;
    game.inventory[..=start_index]
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(index, item)| item.as_ref().map(|unwrapped_item| (index, unwrapped_item)))
        .find(|(i, _)| !calculated_selected_item_indexes.contains(i))
        .map(|(index, item)| handle_conditions(index, error_conditions, item))
        .ok_or_else(|| {
            MyError::create_execute_command_error(format!(
                "index_specifier: {index_specifier:?} did not find any items in inventory from relative point {start_index} \
                 until start of inventory."
            ))
        })?
}
