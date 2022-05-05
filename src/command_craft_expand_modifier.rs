use std::collections::HashMap;
use crate::Game;
use crate::item::Item;
use crate::roll_modifier::execute_craft_roll_modifier;
use serde::{Deserialize, Serialize};
use crate::treasure_types::{TreasureType};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandModifiersReport {
    new_item: Item,
    paid_cost: usize,
    new_cost: usize,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_craft_expand_modifiers(game: &mut Game, inventory_index: usize, mut sacrifice_item_indexes: Vec<usize>) -> Result<ExecuteExpandModifiersReport, String> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len()));
    }
    if game.inventory[inventory_index].is_none() {
        return Err(format!("inventory_index {} is empty.", inventory_index));
    }
    let inventory_item = game.inventory[inventory_index].as_ref().unwrap();
    if usize::from(inventory_item.crafting_info.possible_rolls.min_simultaneous_resistances) <= inventory_item.modifiers.len() {
        return Err(format!("inventory_index.possible_rolls.min_simultaneous_resistances {} need to be bigger than inventory_index current number of modifiers {} for it to be expanded.",
                           inventory_item.crafting_info.possible_rolls.min_simultaneous_resistances,
                           inventory_item.modifiers.len())
        );
    }

    //TODO list of sacrificed items cannot be the same.
    let cost = execute_craft_expand_modifiers_calculate_cost(game, inventory_index);
    if sacrifice_item_indexes.len() < cost.into() {
        return Err(format!("craft_reroll_modifier needs {} items to be sacrificed but you only provided {}", cost, sacrifice_item_indexes.len()));
    }

    //Only need to cost amount of items
    sacrifice_item_indexes.truncate(cost);

    for sacrifice_item_index in sacrifice_item_indexes.clone() {
        if game.inventory.len() <= sacrifice_item_index {
            return Err(format!("sacrifice_item_index {} is not within the range of the inventory {}", sacrifice_item_index, game.inventory.len()));
        }
        if inventory_index == sacrifice_item_index {
            return Err(format!("inventory_index {} and sacrifice_item_index {} cannot be the same", inventory_index, sacrifice_item_index));
        }
        if game.inventory[sacrifice_item_index].is_none() {
            return Err(format!("sacrifice_item_index {} is empty.", sacrifice_item_index));
        }
        let sacrificed_item = game.inventory[sacrifice_item_index].as_ref().unwrap();
        if sacrificed_item.modifiers.len() < inventory_item.modifiers.len() {
            return Err(format!("sacrifice_item_index {} need to have at least {} modifiers but it only had {}", sacrifice_item_index, inventory_item.modifiers.len(), sacrificed_item.modifiers.len()));
        }
    }

    //Crafting cost

    for sacrifice_item_index in sacrifice_item_indexes {
        game.inventory[sacrifice_item_index] = None;
    }

    //Create item
    let new_item_modifier = execute_craft_roll_modifier(game, inventory_index);
    game.inventory[inventory_index].as_mut().unwrap().modifiers.push(new_item_modifier);

    Ok(ExecuteExpandModifiersReport {
        new_item: game.inventory[inventory_index].as_ref().unwrap().clone(),
        paid_cost: cost.clone(),
        new_cost: execute_craft_expand_modifiers_calculate_cost(game, inventory_index),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_craft_expand_modifiers_calculate_cost(game: &Game, inventory_index: usize) -> usize {
    game.inventory[inventory_index].as_ref().unwrap().modifiers.len() * 2
}

#[cfg(test)]
mod tests_int {
    use crate::command_craft_expand_modifier::execute_craft_expand_modifiers;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_modifiers() {
        let mut game = generate_testing_game(Some([1; 16]));
        assert_eq!(1, game.inventory[0].as_ref().unwrap().modifiers.len());

        assert_eq!(Err("craft_reroll_modifier needs 2 items to be sacrificed but you only provided 1".to_string()), execute_craft_expand_modifiers(&mut game, 0, vec![0]));
        assert_eq!(1, game.inventory[0].as_ref().unwrap().modifiers.len());


        assert_eq!(Err("inventory_index 0 and sacrifice_item_index 0 cannot be the same".to_string()), execute_craft_expand_modifiers(&mut game, 0, vec![0, 1]));
        assert_eq!(1, game.inventory[0].as_ref().unwrap().modifiers.len());

        let old_item = game.inventory[0].clone();

        let result = execute_craft_expand_modifiers(&mut game, 0, vec![1, 2]);

        assert!(result.is_ok());
        assert_ne!(old_item.unwrap(), result.unwrap().new_item);
        assert_eq!(2, game.inventory[0].as_ref().unwrap().modifiers.len());

        let old_item = game.inventory[0].clone();

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 9".to_string()), execute_craft_expand_modifiers(&mut game, 99, vec![1, 2]));
        assert_eq!(Err("sacrifice_item_index 99 is not within the range of the inventory 9".to_string()), execute_craft_expand_modifiers(&mut game, 0, vec![99, 1,2,3]));
        assert_eq!(Err("sacrifice_item_index 1 is empty.".to_string()), execute_craft_expand_modifiers(&mut game, 0, vec![1, 2,3,4]));
        assert_eq!(Err("sacrifice_item_index 3 need to have at least 2 modifiers but it only had 1".to_string()), execute_craft_expand_modifiers(&mut game, 0, vec![3,4, 5, 6]));

        //TODO More tests.

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(2, game.inventory[0].as_ref().unwrap().modifiers.len());

        game.inventory[0].as_mut().unwrap().crafting_info.possible_rolls.min_simultaneous_resistances = 0;
        assert_eq!(Err("inventory_index.possible_rolls.min_simultaneous_resistances 0 need to be bigger than inventory_index current number of modifiers 2 for it to be expanded.".to_string()), execute_craft_expand_modifiers(&mut game, 0, vec![1, 2]));
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_craft_expand_modifiers(&mut game, 0, vec![1, 2]);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_craft_expand_modifiers(&mut game, 0, vec![1, 2]);
            assert_eq!(original_result, result);
        }
    }
}