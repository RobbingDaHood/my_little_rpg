use crate::command_create_new_item::execute_create_item;
use crate::Game;
use crate::item::Item;
use crate::treasure_types::TreasureType::Gold;

pub fn execute_expand_equipment_slots(game: &mut Game) -> Result<Item, String> {
    //Crafting cost
    let crafting_cost = (game.equipped_items.len() + 1).pow(5) as u64;
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_equipment_slots, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Pick first item in inventory or
    let item = if game.inventory.len() > 0 {
        game.inventory.remove(game.inventory.len() - 1)
    } else {
        execute_create_item(game)
    };

    game.equipped_items.push(item.clone());

    Ok(item)
}

#[cfg(test)]
mod tests_int {
    use crate::command_expand_equipment_slots::execute_expand_equipment_slots;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_new_game;

    #[test]
    fn test_execute_expand_equipment_slots() {
        let mut game = generate_new_game();
        assert_eq!(1, game.equipped_items.len());

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_equipment_slots, the cost is 32 and you only have Some(0)".to_string()), execute_expand_equipment_slots(&mut game));

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }

        let result = execute_expand_equipment_slots(&mut game);
        assert!(result.is_ok());
        assert_eq!(2, game.equipped_items.len());

        let result = execute_expand_equipment_slots(&mut game);
        assert!(result.is_ok());
        assert_eq!(3, game.equipped_items.len());

        let result = execute_expand_equipment_slots(&mut game);
        assert!(result.is_ok());
        assert_eq!(4, game.equipped_items.len());

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_equipment_slots, the cost is 3125 and you only have Some(1701)".to_string()), execute_expand_equipment_slots(&mut game));
    }
}