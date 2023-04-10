use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::command::roll_modifier::execute_craft;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::index_specifier::{calculate_absolute_item_indexes, IndexSpecifier};
use crate::the_world::item::Item;
use crate::the_world::treasure_types::TreasureType;

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
        return Err(MyError::create_execute_command_error( format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len())));
    }
    if game.inventory[inventory_index].is_none() {
        return Err(MyError::create_execute_command_error( format!("inventory_index {} is empty.", inventory_index)));
    }
    let inventory_item = game.inventory[inventory_index].as_ref().unwrap();
    if usize::from(inventory_item.crafting_info.possible_rolls.min_simultaneous_resistances) <= inventory_item.modifiers.len() {
        return Err(MyError::create_execute_command_error(format!("inventory_index.possible_rolls.min_simultaneous_resistances {} need to be bigger than inventory_index current number of modifiers {} for it to be expanded.",
                           inventory_item.crafting_info.possible_rolls.min_simultaneous_resistances,
                           inventory_item.modifiers.len()))
        );
    }

    let cost = execute_craft_expand_modifiers_calculate_cost(game, inventory_index);
    if sacrifice_item_indexes.len() < cost {
        return Err(MyError::create_execute_command_error(format!("craft_reroll_modifier needs {} items to be sacrificed but you only provided {}", cost, sacrifice_item_indexes.len())));
    }

    //Only need to cost amount of items
    sacrifice_item_indexes.truncate(cost);

    let calculated_sacrifice_item_indexes = calculate_absolute_item_indexes(game, inventory_index, &sacrifice_item_indexes)?;

    for sacrifice_item_index in calculated_sacrifice_item_indexes.clone() {
        let sacrificed_item = game.inventory[sacrifice_item_index].as_ref().unwrap();
        if sacrificed_item.modifiers.len() < inventory_item.modifiers.len() {
            return Err(MyError::create_execute_command_error(format!("sacrifice_item_index {} need to have at least {} modifiers but it only had {}", sacrifice_item_index, inventory_item.modifiers.len(), sacrificed_item.modifiers.len())));
        }
    }

    //Crafting cost
    for sacrifice_item_index in calculated_sacrifice_item_indexes {
        game.inventory[sacrifice_item_index] = None;
    }

    //Create item
    let new_item_modifier = execute_craft(&mut game.random_generator_state, &game.inventory[inventory_index].as_ref().unwrap().crafting_info);
    game.inventory[inventory_index].as_mut().unwrap().modifiers.push(new_item_modifier);

    Ok(ExecuteExpandModifiersReport {
        new_item: game.inventory[inventory_index].as_ref().unwrap().clone(),
        paid_cost: cost,
        new_cost: execute_craft_expand_modifiers_calculate_cost(game, inventory_index),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_craft_expand_modifiers_calculate_cost(game: &Game, inventory_index: usize) -> usize {
    match game.inventory[inventory_index].clone() {
        Some(item) => item.modifiers.len() * 2,
        None => 0
    }
}

#[cfg(test)]
mod tests_int {
    use crate::command::craft_expand_modifier::execute_craft_expand_modifiers;
    use crate::generator::game::new_testing;
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::index_specifier::IndexSpecifier;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_modifiers_absolute() {
        let mut game = new_testing(Some([1; 16]));
        assert_eq!(1, game.inventory[0].as_ref().unwrap().modifiers.len());

        assert_eq!(Err(MyError::create_execute_command_error("craft_reroll_modifier needs 2 items to be sacrificed but you only provided 1".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(0)]));
        assert_eq!(1, game.inventory[0].as_ref().unwrap().modifiers.len());


        assert_eq!(Err(MyError::create_execute_command_error("inventory_index 0 and index_specifier Absolute(0) cannot be the same".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(0), IndexSpecifier::Absolute(1)]));
        assert_eq!(1, game.inventory[0].as_ref().unwrap().modifiers.len());

        let old_item = game.inventory[0].clone();

        let result = execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2)]);

        assert!(result.is_ok());
        assert_ne!(old_item.unwrap(), result.unwrap().new_item);
        assert_eq!(2, game.inventory[0].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[1].is_none());
        assert!(game.inventory[2].is_none());

        let old_item = game.inventory[0].clone();

        assert_eq!(Err(MyError::create_execute_command_error("inventory_index 99 is not within the range of the inventory 9".to_string())), execute_craft_expand_modifiers(&mut game, 99, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2)]));
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier Absolute(99) is not within the range of the inventory 9".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(99), IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2), IndexSpecifier::Absolute(3)]));
        assert_eq!(Err(MyError::create_execute_command_error("inventory_index 1 is empty.".to_string())), execute_craft_expand_modifiers(&mut game, 1, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2), IndexSpecifier::Absolute(3), IndexSpecifier::Absolute(4)]));
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier Absolute(1) is pointing at empty inventory slot.".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2), IndexSpecifier::Absolute(3), IndexSpecifier::Absolute(4)]));
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier Absolute(3) is already present in calculated sacrifice indexes [3]".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(3), IndexSpecifier::Absolute(3), IndexSpecifier::Absolute(3), IndexSpecifier::Absolute(4)]));
        assert_eq!(Err(MyError::create_execute_command_error("sacrifice_item_index 3 need to have at least 2 modifiers but it only had 1".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(3), IndexSpecifier::Absolute(4), IndexSpecifier::Absolute(5), IndexSpecifier::Absolute(6)]));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(2, game.inventory[0].as_ref().unwrap().modifiers.len());

        game.inventory[0].as_mut().unwrap().crafting_info.possible_rolls.min_simultaneous_resistances = 0;
        assert_eq!(Err(MyError::create_execute_command_error("inventory_index.possible_rolls.min_simultaneous_resistances 0 need to be bigger than inventory_index current number of modifiers 2 for it to be expanded.".to_string())), execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2)]));
    }

    #[test]
    fn test_execute_expand_modifiers_relative_positive() {
        let mut game = new_testing(Some([1; 16]));

        let result = execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativePositive(2)]);

        assert!(result.is_ok());
        assert_eq!(2, game.inventory[0].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[1].is_none());
        assert!(game.inventory[2].is_none());

        let result = execute_craft_expand_modifiers(&mut game, 3, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativePositive(1)]);

        assert!(result.is_ok());
        assert_eq!(2, game.inventory[3].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[4].is_none());
        assert!(game.inventory[5].is_none());

        game.inventory[7] = None;
        game.inventory[8] = None;
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier: RelativePositive(1) did not find any items in inventory from relative point 7 until end of inventory.".to_string())), execute_craft_expand_modifiers(&mut game, 6, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativePositive(1)]));
    }

    #[test]
    fn test_execute_expand_modifiers_relative_negative() {
        let mut game = new_testing(Some([1; 16]));

        let result = execute_craft_expand_modifiers(&mut game, 8, vec![IndexSpecifier::RelativeNegative(1), IndexSpecifier::RelativeNegative(2)]);

        assert!(result.is_ok());
        assert_eq!(2, game.inventory[8].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[7].is_none());
        assert!(game.inventory[6].is_none());

        let result = execute_craft_expand_modifiers(&mut game, 5, vec![IndexSpecifier::RelativeNegative(1), IndexSpecifier::RelativeNegative(1)]);

        assert!(result.is_ok());
        assert_eq!(2, game.inventory[5].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[4].is_none());
        assert!(game.inventory[3].is_none());

        game.inventory[1] = None;
        game.inventory[0] = None;
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier: RelativeNegative(1) did not find any items in inventory from relative point 3 until start of inventory.".to_string())), execute_craft_expand_modifiers(&mut game, 2, vec![IndexSpecifier::RelativeNegative(1), IndexSpecifier::RelativeNegative(1)]));
    }

    #[test]
    fn test_execute_expand_modifiers_relative_mix() {
        let mut game = new_testing(Some([1; 16]));

        let result = execute_craft_expand_modifiers(&mut game, 5, vec![IndexSpecifier::RelativeNegative(1), IndexSpecifier::RelativePositive(1)]);

        assert!(result.is_ok());
        assert_eq!(2, game.inventory[5].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[4].is_none());
        assert!(game.inventory[6].is_none());

        let result = execute_craft_expand_modifiers(&mut game, 7, vec![IndexSpecifier::Absolute(3), IndexSpecifier::RelativeNegative(1)]);

        assert!(result.is_ok());
        assert_eq!(2, game.inventory[7].as_ref().unwrap().modifiers.len());
        assert!(game.inventory[3].is_none());
        assert!(game.inventory[5].is_none());

        game.inventory[7] = None;
        game.inventory[1] = None;
        game.inventory[0] = None;
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier: RelativeNegative(1) did not find any items in inventory from relative point 3 until start of inventory.".to_string())), execute_craft_expand_modifiers(&mut game, 2, vec![IndexSpecifier::RelativeNegative(1), IndexSpecifier::RelativePositive(1)]));
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier: RelativeNegative(1) did not find any items in inventory from relative point 3 until start of inventory.".to_string())), execute_craft_expand_modifiers(&mut game, 2, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativeNegative(1)]));

        game.inventory[8] = None;
        assert_eq!(Err(MyError::create_execute_command_error("index_specifier: RelativePositive(1) did not find any items in inventory from relative point 3 until end of inventory.".to_string())), execute_craft_expand_modifiers(&mut game, 2, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativeNegative(1)]));
    }

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2)]);

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_craft_expand_modifiers(&mut game, 0, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2)]);
            assert_eq!(original_result, result);
        }
    }
}