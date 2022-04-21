use std::collections::HashMap;
use crate::Game;
use crate::item::Item;
use crate::roll_modifier::execute_craft_roll_modifier;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::treasure_types::TreasureType;

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
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_modifiers, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Create item
    let new_item_modifier = execute_craft_roll_modifier(game);
    game.inventory[inventory_index].modifiers.push(new_item_modifier);

    Ok(ExecuteExpandModifiersReport {
        new_item: game.inventory[inventory_index].clone(),
        paid_cost: HashMap::from([(Gold, crafting_cost.clone())]),
        new_cost: HashMap::from([(Gold, execute_expand_modifiers_calculate_cost(game, inventory_index))]),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_modifiers_calculate_cost(game: &mut Game, inventory_index: usize) -> u64 {
    (game.inventory[inventory_index].modifiers.len().pow(5) + 10) as u64
}


#[cfg(test)]
mod tests_int {
    use crate::command_expand_modifier::execute_expand_modifiers;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_modifiers() {
        let mut game = generate_testing_game();
        assert_eq!(1, game.inventory[0].modifiers.len());

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_modifiers, the cost is 11 and you only have Some(0)".to_string()), execute_expand_modifiers(&mut game, 0));

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

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 9".to_string()), execute_expand_modifiers(&mut game, 99));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(old_gold, *game.treasure.get(&Gold).unwrap());
        assert_eq!(2, game.inventory[0].modifiers.len());
    }
}