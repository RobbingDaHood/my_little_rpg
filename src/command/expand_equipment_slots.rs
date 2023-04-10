use std::collections::HashMap;
use std::mem;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::item::Item;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandEquipmentSlotsReport {
    new_equipped_items: Vec<Item>,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result)
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandEquipmentSlotsReport, MyError> {
    if game.inventory.is_empty() {
        return Err(MyError::create_execute_command_error("No item in inventory to equip in new item slot.".to_string()));
    }

    let first_item_index = match game.inventory.iter()
        .position(Option::is_some) {
        Some(index) => index,
        None => return Err(MyError::create_execute_command_error("No item in inventory to equip in new item slot. Whole inventory is empty.".to_string()))
    };

    //Crafting cost
    let crafting_cost = execute_expand_equipment_slots_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Pick first item in inventory or
    let item = mem::replace(&mut game.inventory[first_item_index], None);
    game.equipped_items.push(item.unwrap());

    Ok(ExecuteExpandEquipmentSlotsReport {
        new_equipped_items: game.equipped_items.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_equipment_slots_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_equipment_slots_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.equipped_items.len() + 1).pow(5) as u64)])
}

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