use std::mem;
use crate::Game;
use serde::{Deserialize, Serialize};
use crate::item::Item;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteEquipOrSwapReport {
    new_equipped_items: Vec<Item>,
}

pub fn execute_equip_item(game: &mut Game, inventory_position: usize, equipped_item_position: usize) -> Result<ExecuteEquipOrSwapReport, String> {
    if game.equipped_items.len() < equipped_item_position {
        return Err(format!("equipped_item_position {} is not within the range of the equipment slots {}", equipped_item_position, game.equipped_items.len()));
    }
    if game.inventory.len() < inventory_position {
        return Err(format!("inventory_position {} is not within the range of the inventory {}", inventory_position, game.inventory.len()));
    }
    if game.inventory[inventory_position].is_none() {
        return Err(format!("inventory_position {} is empty.", inventory_position));
    }

    let inventory_item = mem::replace(&mut game.inventory[inventory_position], Some(game.equipped_items[equipped_item_position].clone()));
    game.equipped_items[equipped_item_position] = inventory_item.unwrap();

    Ok(ExecuteEquipOrSwapReport { new_equipped_items: game.equipped_items.clone() })
}

pub fn execute_swap_equipped_item(game: &mut Game, equipped_item_position_1: usize, equipped_item_position_2: usize) -> Result<ExecuteEquipOrSwapReport, String> {
    if game.equipped_items.len() < equipped_item_position_1 {
        return Err(format!("equipped_item_position_1 {} is not within the range of the equipment slots {}", equipped_item_position_1, game.equipped_items.len()));
    }
    if game.equipped_items.len() < equipped_item_position_2 {
        return Err(format!("equipped_item_position_2 {} is not within the range of the equipment slots {}", equipped_item_position_2, game.equipped_items.len()));
    }
    if equipped_item_position_1 == equipped_item_position_2 {
        return Err(format!("equipped_item_position_1 {} cannot be the same as equipped_item_position_2 {}", equipped_item_position_1, equipped_item_position_2));
    }

    game.equipped_items.swap(equipped_item_position_1, equipped_item_position_2);

    Ok(ExecuteEquipOrSwapReport { new_equipped_items: game.equipped_items.clone() })
}


#[cfg(test)]
mod tests_int {
    use crate::command_equip_swap::{execute_equip_item, execute_swap_equipped_item};
    use crate::game_generator::generate_testing_game;

    #[test]
    fn test_execute_equip_item() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert!(execute_equip_item(&mut game, 0, 0).is_ok());

        assert_eq!(Some(equipped_item), game.inventory[0]);
        assert_eq!(inventory_item, Some(game.equipped_items[0].clone()));
    }

    #[test]
    fn test_execute_equip_item_inventory_out_of_bounds() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(Err("inventory_position 999 is not within the range of the inventory 9".to_string()), execute_equip_item(&mut game, 999, 0));

        assert_eq!(inventory_item, game.inventory[0]);
        assert_eq!(equipped_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_equip_item_equipment_out_of_bounds() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(Err("equipped_item_position 999 is not within the range of the equipment slots 2".to_string()), execute_equip_item(&mut game, 0, 999));

        assert_eq!(inventory_item, game.inventory[0]);
        assert_eq!(equipped_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_swap_equipped_item() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert!(execute_swap_equipped_item(&mut game, 0, 1).is_ok());

        assert_eq!(equipped_item_2, game.equipped_items[0]);
        assert_eq!(equipped_item_1, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_1_out_of_bounds() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Err("equipped_item_position_1 999 is not within the range of the equipment slots 2".to_string()), execute_swap_equipped_item(&mut game, 999, 1));

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_2_out_of_bounds() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Err("equipped_item_position_2 999 is not within the range of the equipment slots 2".to_string()), execute_swap_equipped_item(&mut game, 0, 999));

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_1_and_2_are_the_same() {
        let mut game = generate_testing_game(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Err("equipped_item_position_1 0 cannot be the same as equipped_item_position_2 0".to_string()), execute_swap_equipped_item(&mut game, 0, 0));

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }
}