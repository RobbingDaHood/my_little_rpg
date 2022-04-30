use std::collections::HashMap;
use crate::Game;
use crate::item::Item;
use crate::treasure_types::TreasureType::Gold;
use crate::roll_modifier::execute_craft_roll_modifier;
use crate::treasure_types::{pay_crafting_cost, TreasureType};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteCraftRerollModifierReport {
    new_item: Item,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_craft_reroll_modifier(game: &mut Game, inventory_index: usize, modifier_index: usize) -> Result<ExecuteCraftRerollModifierReport, String> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len()));
    }
    if game.inventory[inventory_index].modifiers.len() <= modifier_index {
        return Err(format!("modifier_index {} is not within the range of the item modifiers {}", inventory_index, game.inventory[inventory_index].modifiers.len()));
    }

    //Crafting cost
    let crafting_cost = execute_craft_reroll_modifier_calculate_cost(game, inventory_index, modifier_index);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message)
    };

    //Create item
    let new_item_modifier = execute_craft_roll_modifier(game);
    game.inventory[inventory_index].modifiers[modifier_index] = new_item_modifier;

    Ok(ExecuteCraftRerollModifierReport {
        new_item: game.inventory[inventory_index].clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_craft_reroll_modifier_calculate_cost(game, inventory_index, modifier_index),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_craft_reroll_modifier_calculate_cost(game: &Game, inventory_index: usize, modifier_index: usize) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.inventory[inventory_index].modifiers.len() * (modifier_index + 1) * 5) as u64)])
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;
    use crate::command_craft_reroll_modifier::{execute_craft_reroll_modifier};
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_craft_item() {
        let mut game = generate_testing_game(Some([1; 16]));

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 5} and you only have {}".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0));

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);

        let old_item = game.inventory[0].clone();

        let result = execute_craft_reroll_modifier(&mut game, 0, 0);

        assert!(result.is_ok());
        assert_ne!(old_item, result.clone().unwrap().new_item);
        assert_eq!(HashMap::from([(Gold, 5)]), result.clone().unwrap().paid_cost);
        assert!(result.clone().unwrap().leftover_spending_treasure.get(&Gold).unwrap() > &0);

        let old_item = game.inventory[0].clone();
        let old_gold = game.treasure.get(&Gold).unwrap().clone();

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 9".to_string()), execute_craft_reroll_modifier(&mut game, 99, 0));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(old_gold, *game.treasure.get(&Gold).unwrap());

        assert_eq!(Err("modifier_index 0 is not within the range of the item modifiers 1".to_string()), execute_craft_reroll_modifier(&mut game, 0, 99));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(old_gold, *game.treasure.get(&Gold).unwrap());
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_craft_reroll_modifier(&mut game, 0, 0);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_craft_reroll_modifier(&mut game, 0, 0);
            assert_eq!(original_result, result);
        }
    }
}