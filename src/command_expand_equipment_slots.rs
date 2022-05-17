use std::collections::HashMap;
use std::mem;
use crate::Game;
use crate::item::Item;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::treasure_types::{pay_crafting_cost, TreasureType};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandEquipmentSlotsReport {
    new_equipped_items: Vec<Item>,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_equipment_slots(game: &mut Game) -> Result<ExecuteExpandEquipmentSlotsReport, String> {
    if game.inventory.is_empty() {
        return Err("No item in inventory to equip in new item slot.".to_string());
    }

    let first_item_index = match game.inventory.iter()
        .position(|item| item.is_some()) {
        Some(index) => index,
        None => return Err("No item in inventory to equip in new item slot. Whole inventory is empty.".to_string())
    };

    //Crafting cost
    let crafting_cost = execute_expand_equipment_slots_calculate_cost(game);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message);
    };

    //Pick first item in inventory or
    let item = mem::replace(&mut game.inventory[first_item_index], None);
    game.equipped_items.push(item.unwrap().clone());

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
    use crate::command_expand_equipment_slots::execute_expand_equipment_slots;
    use crate::game_generator::generate_new_game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::ItemModifier;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_equipment_slots() {
        let mut game = generate_new_game(Some([1; 16]));
        assert_eq!(1, game.equipped_items.len());

        assert_eq!(Err("No item in inventory to equip in new item slot.".to_string()), execute_expand_equipment_slots(&mut game));
        assert_eq!(1, game.equipped_items.len());

        let item = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };
        game.inventory.push( Some(item.clone()));
        game.inventory.push( Some(item.clone()));
        game.inventory.push( Some(item.clone()));

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 32} and you only have {}".to_string()), execute_expand_equipment_slots(&mut game));

        game.treasure.insert(Gold, 1300);
        let result = execute_expand_equipment_slots(&mut game);
        assert!(result.is_ok());
        assert_eq!(2, game.equipped_items.len());

        let result = execute_expand_equipment_slots(&mut game);
        assert!(result.is_ok());
        assert_eq!(3, game.equipped_items.len());

        let result = execute_expand_equipment_slots(&mut game);
        assert!(result.is_ok());
        assert_eq!(4, game.equipped_items.len());

        assert_eq!(Err("No item in inventory to equip in new item slot. Whole inventory is empty.".to_string()), execute_expand_equipment_slots(&mut game));
        game.inventory.push( Some(item.clone()));
        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 3125} and you only have {Gold: 1}".to_string()), execute_expand_equipment_slots(&mut game));
    }
}