use serde::{Deserialize, Serialize};

use crate::Game;
use crate::index_specifier::{calculate_absolute_item_indexes, IndexSpecifier};
use crate::item::Item;
use crate::roll_modifier::execute_craft;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteCraftRerollModifierReport {
    new_item: Item,
    paid_cost: u16,
    new_cost: u16,
}

pub fn execute_craft_reroll_modifier(game: &mut Game, inventory_index: usize, modifier_index: usize, mut sacrifice_item_indexes: Vec<IndexSpecifier>) -> Result<ExecuteCraftRerollModifierReport, String> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len()));
    }
    if game.inventory[inventory_index].is_none() {
        return Err(format!("inventory_index {} is empty.", inventory_index));
    }
    let inventory_item = game.inventory[inventory_index].as_ref().unwrap();
    if inventory_item.modifiers.len() <= modifier_index {
        return Err(format!("modifier_index {} is not within the range of the item modifiers {}", modifier_index, inventory_item.modifiers.len()));
    }

    //Crafting cost
    let cost = execute_craft_reroll_modifier_calculate_cost(game, inventory_index);
    if sacrifice_item_indexes.len() < cost.into() {
        return Err(format!("craft_reroll_modifier needs {} items to be sacrificed but you only provided {}", cost, sacrifice_item_indexes.len()));
    }

    //Only need to cost amount of items
    sacrifice_item_indexes.truncate(usize::from(cost));

    let calculated_sacrifice_item_indexes = match calculate_absolute_item_indexes(game, inventory_index, &sacrifice_item_indexes) {
        Err(error_message) => return Err(error_message),
        Ok(indexes) => indexes
    };

    for sacrifice_item_index in calculated_sacrifice_item_indexes.clone() {
        let sacrificed_item = game.inventory[sacrifice_item_index].as_ref().unwrap();
        if sacrificed_item.modifiers.len() <= modifier_index {
            return Err(format!("sacrifice_item_index {} need to have at least {} modifiers but it only had {}", sacrifice_item_index, modifier_index + 1, sacrificed_item.modifiers.len()));
        }
    }

    //Crafting cost
    for sacrifice_item_index in calculated_sacrifice_item_indexes {
        game.inventory[sacrifice_item_index] = None;
    }

    //Create item
    let new_item_modifier = execute_craft(&mut game.random_generator_state, &game.inventory[inventory_index].as_ref().unwrap().crafting_info);
    game.inventory[inventory_index].as_mut().unwrap().modifiers[modifier_index] = new_item_modifier;

    Ok(ExecuteCraftRerollModifierReport {
        new_item: game.inventory[inventory_index].as_ref().unwrap().clone(),
        new_cost: execute_craft_reroll_modifier_calculate_cost(game, inventory_index),
        paid_cost: cost,
    })
}

pub fn execute_craft_reroll_modifier_calculate_cost(game: &Game, inventory_index: usize) -> u16 {
    match game.inventory[inventory_index].clone() {
        #[allow(clippy::cast_possible_truncation)]
        Some(item) => item.modifiers.len() as u16,
        None => 0
    }
}

#[cfg(test)]
mod tests_int {
    use crate::{Game, index_specifier};
    use crate::command_craft_reroll_modifier::{execute_craft_reroll_modifier, execute_craft_reroll_modifier_calculate_cost};
    use crate::game_generator::generate_testing_game;
    use crate::item::test_util::create_item;

    #[test]
    fn test_execute_craft_item() {
        let mut game = generate_testing_game(Some([1; 16]));

        insert_game_in_inventory(&mut game);

        let old_item = game.inventory[0].clone();
        assert_eq!(10, game.inventory.len());

        assert_eq!(Err("inventory_index 0 and index_specifier Absolute(0) cannot be the same".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(0), index_specifier::IndexSpecifier::Absolute(1)]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        assert_eq!(Err("sacrifice_item_index 1 need to have at least 2 modifiers but it only had 1".to_string()), execute_craft_reroll_modifier(&mut game, 0, 1, vec![index_specifier::IndexSpecifier::Absolute(1), index_specifier::IndexSpecifier::Absolute(2)]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        assert_eq!(2, execute_craft_reroll_modifier_calculate_cost(&game, 0));
        assert_eq!(Err("craft_reroll_modifier needs 2 items to be sacrificed but you only provided 1".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(1)]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(1), index_specifier::IndexSpecifier::Absolute(2)]);
        assert!(result.is_ok());
        assert_ne!(old_item.unwrap(), result.unwrap().new_item);
        let old_item = game.inventory[0].clone();
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 10".to_string()), execute_craft_reroll_modifier(&mut game, 99, 0, vec![index_specifier::IndexSpecifier::Absolute(0)]));
        assert_eq!(Err("modifier_index 99 is not within the range of the item modifiers 2".to_string()), execute_craft_reroll_modifier(&mut game, 0, 99, vec![index_specifier::IndexSpecifier::Absolute(0)]));
        assert_eq!(Err("index_specifier Absolute(99) is not within the range of the inventory 10".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(99), index_specifier::IndexSpecifier::Absolute(1)]));
        assert_eq!(Err("inventory_index 1 is empty.".to_string()), execute_craft_reroll_modifier(&mut game, 1, 0, vec![index_specifier::IndexSpecifier::Absolute(4), index_specifier::IndexSpecifier::Absolute(5)]));
        assert_eq!(Err("index_specifier Absolute(1) is pointing at empty inventory slot.".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(1), index_specifier::IndexSpecifier::Absolute(2)]));
        assert_eq!(Err("index_specifier Absolute(8) is already present in calculated sacrifice indexes [8]".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(8), index_specifier::IndexSpecifier::Absolute(8)]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());
    }

    #[test]
    fn test_execute_craft_item_positive() {
        let mut game = generate_testing_game(Some([1; 16]));

        insert_game_in_inventory(&mut game);

        let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativePositive(2)]);
        assert!(result.is_ok());
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativePositive(1)]);
        assert!(result.is_ok());
        assert_eq!(6, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativePositive(1)]);
        assert!(result.is_ok());
        assert_eq!(4, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativePositive(1)]);
        assert!(result.is_ok());
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(Err("index_specifier: RelativePositive(1) did not find any items in inventory from relative point 1 until end of inventory.".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativePositive(1)]));
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());
    }

    fn insert_game_in_inventory(game: &mut Game) {
        game.inventory.insert(0, Some(create_item(game)));
    }

    #[test]
    fn test_execute_craft_item_negative() {
        let mut game = generate_testing_game(Some([1; 16]));

        game.inventory.push(Some(create_item(&game)));

        let result = execute_craft_reroll_modifier(&mut game, 9, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativeNegative(2)]);
        assert!(result.is_ok());
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 9, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativeNegative(1)]);
        assert!(result.is_ok());
        assert_eq!(6, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 9, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativeNegative(1)]);
        assert!(result.is_ok());
        assert_eq!(4, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 9, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativeNegative(1)]);
        assert!(result.is_ok());
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(Err("index_specifier: RelativeNegative(1) did not find any items in inventory from relative point 10 until start of inventory.".to_string()), execute_craft_reroll_modifier(&mut game, 9, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativeNegative(1)]));
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());
    }

    #[test]
    fn test_execute_craft_item_mixed() {
        let mut game = generate_testing_game(Some([1; 16]));

        game.inventory.insert(5, Some(create_item(&game)));

        let result = execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativePositive(2)]);
        assert!(result.is_ok());
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativeNegative(1)]);
        assert!(result.is_ok());
        assert_eq!(6, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::Absolute(0)]);
        assert!(result.is_ok());
        assert_eq!(4, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::Absolute(9)]);
        assert!(result.is_ok());
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(Err("index_specifier: RelativePositive(1) did not find any items in inventory from relative point 6 until end of inventory.".to_string()), execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativePositive(1), index_specifier::IndexSpecifier::RelativeNegative(1)]));
        assert_eq!(Err("index_specifier: RelativePositive(1) did not find any items in inventory from relative point 6 until end of inventory.".to_string()), execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativePositive(1)]));
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        game.inventory[2] = None;
        assert_eq!(Err("index_specifier: RelativeNegative(1) did not find any items in inventory from relative point 6 until start of inventory.".to_string()), execute_craft_reroll_modifier(&mut game, 5, 0, vec![index_specifier::IndexSpecifier::RelativeNegative(1), index_specifier::IndexSpecifier::RelativeNegative(1)]));
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        let original_result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(1)]);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(1)]);
            assert_eq!(original_result, result);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = generate_testing_game(Some([1; 16]));

        for i in 1..438 {
            game.inventory.push(Some(create_item(&game)));
            assert!(execute_craft_reroll_modifier(&mut game, 0, 0, vec![index_specifier::IndexSpecifier::Absolute(i)]).is_ok());
        }
    }
}