use std::collections::HashMap;
use crate::Game;
use crate::item::Item;
use crate::roll_modifier::execute_craft_roll_modifier;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::treasure_types::{pay_crafting_cost, TreasureType};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandModifiersReport {
    new_item: Item,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_modifiers(game: &mut Game, inventory_index: usize) -> Result<ExecuteExpandModifiersReport, String> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len()));
    }

    //Crafting cost
    let crafting_cost = execute_expand_modifiers_calculate_cost(game, inventory_index);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message)
    };

    //Create item
    let new_item_modifier = execute_craft_roll_modifier(game);
    game.inventory[inventory_index].modifiers.push(new_item_modifier);

    Ok(ExecuteExpandModifiersReport {
        new_item: game.inventory[inventory_index].clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_modifiers_calculate_cost(game, inventory_index),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_modifiers_calculate_cost(game: &Game, inventory_index: usize) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.inventory[inventory_index].modifiers.len().pow(5) + 10) as u64)])
}


#[cfg(test)]
mod tests_int {
    use crate::command_expand_modifier::execute_expand_modifiers;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_modifiers() {
        let mut game = generate_testing_game(Some([1; 16]));
        assert_eq!(1, game.inventory[0].modifiers.len());

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 11} and you only have {}".to_string()), execute_expand_modifiers(&mut game, 0));

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.inventory[0].modifiers.len());

        let old_item = game.inventory[0].clone();

        let result = execute_expand_modifiers(&mut game, 0);

        assert!(result.is_ok());
        assert_ne!(old_item, result.unwrap().new_item);
        assert_eq!(2, game.inventory[0].modifiers.len());

        let old_item = game.inventory[0].clone();
        let old_gold = game.treasure.get(&Gold).unwrap().clone();

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 10".to_string()), execute_expand_modifiers(&mut game, 99));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(old_gold, *game.treasure.get(&Gold).unwrap());
        assert_eq!(2, game.inventory[0].modifiers.len());
    }

    #[test]
    fn test_that_all_elements_can_be_hit() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 999999);

        for _i in 0..12 {
            assert!(execute_expand_modifiers(&mut game, 0).is_ok());
        }
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_expand_modifiers(&mut game, 0);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_expand_modifiers(&mut game, 0);
            assert_eq!(original_result, result);
        }
    }
}