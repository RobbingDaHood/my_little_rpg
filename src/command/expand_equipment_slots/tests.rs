#[cfg(test)]
mod tests_int {
    use crate::command::expand_equipment_slots::execute;
    use crate::generator::game::new;
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::item::test_util::create_item;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_equipment_slots() {
        let mut game = new(Some([1; 16]));
        assert_eq!(1, game.equipped_items.len());

        assert_eq!(Err(MyError::create_execute_command_error("No item in inventory to equip in new item slot.".to_string())), execute(&mut game));
        assert_eq!(1, game.equipped_items.len());

        let item = create_item(&game);
        game.inventory.push(Some(item.clone()));
        game.inventory.push(Some(item.clone()));
        game.inventory.push(Some(item.clone()));

        assert_eq!(Err(MyError::create_execute_command_error("Cant pay the crafting cost, the cost is {Gold: 32} and you only have {}".to_string())), execute(&mut game));

        game.treasure.insert(Gold, 1300);
        let result = execute(&mut game);
        assert!(result.is_ok());
        assert_eq!(2, game.equipped_items.len());

        let result = execute(&mut game);
        assert!(result.is_ok());
        assert_eq!(3, game.equipped_items.len());

        let result = execute(&mut game);
        assert!(result.is_ok());
        assert_eq!(4, game.equipped_items.len());

        assert_eq!(Err(MyError::create_execute_command_error("No item in inventory to equip in new item slot. Whole inventory is empty.".to_string())), execute(&mut game));
        game.inventory.push(Some(item));
        assert_eq!(Err(MyError::create_execute_command_error("Cant pay the crafting cost, the cost is {Gold: 3125} and you only have {Gold: 1}".to_string())), execute(&mut game));
    }
}
