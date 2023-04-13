#[cfg(test)]
mod tests_int {
    use crate::{
        command::equip_swap::{execute_equip_item, execute_swap_equipped_item},
        generator::game::new_testing,
        my_little_rpg_errors::MyError,
    };

    #[test]
    fn test_execute_equip_item() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert!(execute_equip_item(&mut game, 0, 0).is_ok());

        assert_eq!(Some(equipped_item), game.inventory[0]);
        assert_eq!(inventory_item, Some(game.equipped_items[0].clone()));
    }

    #[test]
    fn test_execute_equip_item_inventory_out_of_bounds() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "inventory_position 999 is not within the range of the inventory 9".to_string()
            )),
            execute_equip_item(&mut game, 999, 0)
        );

        assert_eq!(inventory_item, game.inventory[0]);
        assert_eq!(equipped_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_equip_item_equipment_out_of_bounds() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item = game.equipped_items[0].clone();
        let inventory_item = game.inventory[0].clone();

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "equipped_item_position 999 is not within the range of the equipment slots 2"
                    .to_string()
            )),
            execute_equip_item(&mut game, 0, 999)
        );

        assert_eq!(inventory_item, game.inventory[0]);
        assert_eq!(equipped_item, game.equipped_items[0]);
    }

    #[test]
    fn test_execute_swap_equipped_item() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert!(execute_swap_equipped_item(&mut game, 0, 1).is_ok());

        assert_eq!(equipped_item_2, game.equipped_items[0]);
        assert_eq!(equipped_item_1, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_1_out_of_bounds() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "equipped_item_position_1 999 is not within the range of the equipment slots 2"
                    .to_string()
            )),
            execute_swap_equipped_item(&mut game, 999, 1)
        );

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_2_out_of_bounds() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "equipped_item_position_2 999 is not within the range of the equipment slots 2"
                    .to_string()
            )),
            execute_swap_equipped_item(&mut game, 0, 999)
        );

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }

    #[test]
    fn test_execute_swap_equipped_item_equipped_item_1_and_2_are_the_same() {
        let mut game = new_testing(Some([1; 16]));

        let equipped_item_1 = game.equipped_items[0].clone();
        let equipped_item_2 = game.equipped_items[1].clone();

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "equipped_item_position_1 0 cannot be the same as equipped_item_position_2 0"
                    .to_string()
            )),
            execute_swap_equipped_item(&mut game, 0, 0)
        );

        assert_eq!(equipped_item_1, game.equipped_items[0]);
        assert_eq!(equipped_item_2, game.equipped_items[1]);
    }
}
