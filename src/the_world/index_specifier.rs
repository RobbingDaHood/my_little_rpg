use serde::{Deserialize, Serialize};

use crate::Game;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum IndexSpecifier {
    Absolute(usize),
    RelativePositive(usize),
    RelativeNegative(usize),
}

pub fn calculate_absolute_item_indexes(
    game: &Game,
    inventory_index: usize,
    index_specifiers: &[IndexSpecifier],
) -> Result<Vec<usize>, String> {
    let mut calculated_selected_item_indexes = Vec::new();
    for index_specifier in index_specifiers {
        match index_specifier {
            IndexSpecifier::Absolute(index) => {
                if inventory_index == *index {
                    return Err(format!("inventory_index {} and index_specifier {:?} cannot be the same", inventory_index, index_specifier));
                }
                if game.inventory.len() <= *index {
                    return Err(format!("index_specifier {:?} is not within the range of the inventory {}", index_specifier, game.inventory.len()));
                }
                if game.inventory[*index].is_none() {
                    return Err(format!("index_specifier {:?} is pointing at empty inventory slot.", index_specifier));
                };
                if calculated_selected_item_indexes.contains(index) {
                    return Err(format!("index_specifier {:?} is already present in calculated sacrifice indexes {:?}", index_specifier, calculated_selected_item_indexes));
                }
                calculated_selected_item_indexes.push(*index);
            }
            IndexSpecifier::RelativePositive(relative_index) => {
                if inventory_index + relative_index >= game.inventory.len() {
                    return Err(format!("index_specifier: {:?} and {} is outside of the length of the inventory {}", index_specifier, inventory_index, game.inventory.len()));
                };

                let mut index = None;
                for i in inventory_index + relative_index..game.inventory.len() {
                    if game.inventory[i].is_some() && !calculated_selected_item_indexes.contains(&i) {
                        index = Some(i);
                        break;
                    }
                }

                match index {
                    None => return Err(format!("index_specifier: {:?} did not find any items in inventory from relative point {} until end of inventory.", index_specifier, inventory_index + relative_index)),
                    Some(i) => calculated_selected_item_indexes.push(i)
                };
            }
            IndexSpecifier::RelativeNegative(relative_index) => {
                let mut index = None;
                for i in (0..=inventory_index - relative_index).rev() {
                    if game.inventory[i].is_some() && !calculated_selected_item_indexes.contains(&i) {
                        index = Some(i);
                        break;
                    }
                }

                match index {
                    None => return Err(format!("index_specifier: {:?} did not find any items in inventory from relative point {} until start of inventory.", index_specifier, inventory_index + relative_index)),
                    Some(i) => calculated_selected_item_indexes.push(i)
                };
            }
        }
    }
    Ok(calculated_selected_item_indexes)
}