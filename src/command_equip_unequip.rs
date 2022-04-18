use crate::Game;

pub fn execute_equip_item(game: &mut Game, inventory_position: usize, equipped_item_position: usize) -> Result<String, String> {
    if game.equipped_items.len() < equipped_item_position {
        return Err(format!("equipped_item_position {} is not within the range of the equipment slots {}", equipped_item_position, game.equipped_items.len()));
    }
    if game.inventory.len() < inventory_position {
        return Err(format!("inventory_position {} is not within the range of the inventory {}", inventory_position, game.inventory.len()));
    }

    game.inventory.push(game.equipped_items.remove(equipped_item_position));
    game.equipped_items.insert(equipped_item_position, game.inventory.remove(inventory_position));

    Ok("Equipped item.".to_string())
}

pub fn execute_swap_equipped_item(game: &mut Game, equipped_item_position_1: usize, equipped_item_position_2: usize) -> Result<String, String> {
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

    Ok("Swapped equipped item.".to_string())
}


#[cfg(test)]
mod tests_int {
    use crate::command_equip_unequip::{execute_equip_item, execute_swap_equipped_item};
    use crate::game_generator::generate_testing_game;

    #[test]
    fn test_execute_equip_item() {
        let mut game = generate_testing_game();

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(Ok("Equipped item.".to_string()), execute_equip_item(&mut game, 0, 0));

        assert_eq!(&equipped_item, game.inventory.last().unwrap());
        assert_eq!(inventory_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_equip_item_inventory_out_of_bounds() {
        let mut game = generate_testing_game();

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(Err("inventory_position 999 is not within the range of the inventory 9".to_string()), execute_equip_item(&mut game, 999, 0));

        assert_eq!(inventory_item, game.inventory[0]);
        assert_eq!(equipped_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_equip_item_equipment_out_of_bounds() {
        let mut game = generate_testing_game();

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(Err("equipped_item_position 999 is not within the range of the equipment slots 2".to_string()), execute_equip_item(&mut game, 0, 999));

        assert_eq!(inventory_item, game.inventory[0]);
        assert_eq!(equipped_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_swap_equipped_item() {
        let mut game = generate_testing_game();

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Ok("Swapped equipped item.".to_string()), execute_swap_equipped_item(&mut game, 0, 1));

        assert_eq!(equipped_item_2, game.equipped_items[0]);
        assert_eq!(equipped_item_1, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_1_out_of_bounds() {
        let mut game = generate_testing_game();

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Err("equipped_item_position_1 999 is not within the range of the equipment slots 2".to_string()), execute_swap_equipped_item(&mut game, 999, 1));

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_2_out_of_bounds() {
        let mut game = generate_testing_game();

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Err("equipped_item_position_2 999 is not within the range of the equipment slots 2".to_string()), execute_swap_equipped_item(&mut game, 0, 999));

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_1_and_2_are_the_same() {
        let mut game = generate_testing_game();

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(Err("equipped_item_position_1 0 cannot be the same as equipped_item_position_2 0".to_string()), execute_swap_equipped_item(&mut game, 0, 0));

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }
}