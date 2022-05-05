use std::collections::HashSet;
use crate::Game;
use crate::item::Item;
use crate::roll_modifier::execute_craft_roll_modifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteCraftRerollModifierReport {
    new_item: Item,
    paid_cost: u16,
    new_cost: u16,
}

pub fn execute_craft_reroll_modifier(game: &mut Game, inventory_index: usize, modifier_index: usize, mut sacrifice_item_indexes: Vec<usize>) -> Result<ExecuteCraftRerollModifierReport, String> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len()));
    }
    if game.inventory[inventory_index].is_none() {
        return Err(format!("inventory_index {} is empty.", inventory_index));
    }
    let inventory_item = game.inventory[inventory_index].as_ref().unwrap();
    if inventory_item.modifiers.len() <= modifier_index {
        return Err(format!("modifier_index {} is not within the range of the item modifiers {}", modifier_index, inventory_item.modifiers.len()));
    }

    //Crafting cost
    let cost = execute_craft_reroll_modifier_calculate_cost(game, inventory_index);
    if sacrifice_item_indexes.len() < cost.into() {
        return Err(format!("craft_reroll_modifier needs {} items to be sacrificed but you only provided {}", cost, sacrifice_item_indexes.len()));
    }

    let unique_sacrificed_indexes = sacrifice_item_indexes.clone().into_iter().collect::<HashSet<usize>>().len();
    if unique_sacrificed_indexes != sacrifice_item_indexes.len() {
        return Err(format!("sacrifice_item_indexes cannot contain duplicates {:?}", sacrifice_item_indexes));
    }

    //Only need to cost amount of items
    sacrifice_item_indexes.truncate(usize::from(cost));

    for sacrifice_item_index in sacrifice_item_indexes.clone() {
        if game.inventory.len() <= sacrifice_item_index {
            return Err(format!("sacrifice_item_index {} is not within the range of the inventory {}", sacrifice_item_index, game.inventory.len()));
        }
        if inventory_index == sacrifice_item_index {
            return Err(format!("inventory_index {} and sacrifice_item_index {} cannot be the same", inventory_index, sacrifice_item_index));
        }
        if game.inventory[sacrifice_item_index].is_none() {
            return Err(format!("sacrifice_item_index {} is empty.", sacrifice_item_index));
        }
        let sacrificed_item = game.inventory[sacrifice_item_index].as_ref().unwrap();
        if sacrificed_item.modifiers.len() <= modifier_index {
            return Err(format!("sacrifice_item_index {} need to have at least {} modifiers but it only had {}", sacrifice_item_index, modifier_index + 1, sacrificed_item.modifiers.len()));
        }
    }

    for sacrifice_item_index in sacrifice_item_indexes {
        game.inventory[sacrifice_item_index] = None;
    }

    //Create item
    let new_item_modifier = execute_craft_roll_modifier(game, inventory_index);
    game.inventory[inventory_index].as_mut().unwrap().modifiers[modifier_index] = new_item_modifier;

    Ok(ExecuteCraftRerollModifierReport {
        new_item: game.inventory[inventory_index].as_ref().unwrap().clone(),
        new_cost: execute_craft_reroll_modifier_calculate_cost(game, inventory_index),
        paid_cost: cost,
    })
}

pub fn execute_craft_reroll_modifier_calculate_cost(game: &Game, inventory_index: usize) -> u16 {
    return match game.inventory[inventory_index].clone() {
        Some(item) => item.modifiers.len() as u16,
        None => 0
    };
}

#[cfg(test)]
mod tests_int {
    use crate::command_craft_reroll_modifier::{execute_craft_reroll_modifier, execute_craft_reroll_modifier_calculate_cost};
    use crate::game_generator::generate_testing_game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::ItemModifier;

    #[test]
    fn test_execute_craft_item() {
        let mut game = generate_testing_game(Some([1; 16]));

        game.inventory.insert(0, Some(Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                },
                ItemModifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                },
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        }));

        let old_item = game.inventory[0].clone();
        assert_eq!(10, game.inventory.len());

        assert_eq!(Err("inventory_index 0 and sacrifice_item_index 0 cannot be the same".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![0, 1]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        assert_eq!(Err("sacrifice_item_index 1 need to have at least 2 modifiers but it only had 1".to_string()), execute_craft_reroll_modifier(&mut game, 0, 1, vec![1, 2]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        assert_eq!(2, execute_craft_reroll_modifier_calculate_cost(&mut game, 0));
        assert_eq!(Err("craft_reroll_modifier needs 2 items to be sacrificed but you only provided 1".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![1]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![1, 2]);
        assert!(result.is_ok());
        assert_ne!(old_item.unwrap(), result.clone().unwrap().new_item);
        let old_item = game.inventory[0].clone();
        assert_eq!(8, game.inventory.iter().filter(|i|i.is_some()).count());

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 10".to_string()), execute_craft_reroll_modifier(&mut game, 99, 0, vec![0]));
        assert_eq!(Err("modifier_index 99 is not within the range of the item modifiers 2".to_string()), execute_craft_reroll_modifier(&mut game, 0, 99, vec![0]));
        assert_eq!(Err("sacrifice_item_index 99 is not within the range of the inventory 10".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![99, 1]));
        assert_eq!(Err("inventory_index 1 is empty.".to_string()), execute_craft_reroll_modifier(&mut game, 1, 0, vec![4,5]));
        assert_eq!(Err("sacrifice_item_index 1 is empty.".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![1,2]));
        assert_eq!(Err("sacrifice_item_indexes cannot contain duplicates [1, 1]".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0, vec![1,1]));
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(8, game.inventory.iter().filter(|i|i.is_some()).count());
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        let original_result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![1]);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            let result = execute_craft_reroll_modifier(&mut game, 0, 0, vec![1]);
            assert_eq!(original_result, result);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = generate_testing_game(Some([1; 16]));

        for i in 1..438 {
            game.inventory.push(Some(Item {
                modifiers: vec![
                    ItemModifier {
                        costs: Vec::new(),
                        gains: Vec::new(),
                    },
                ],
                crafting_info: CraftingInfo {
                    possible_rolls: game.difficulty.clone()
                },
            }));
            assert!(execute_craft_reroll_modifier(&mut game, 0, 0, vec![i]).is_ok());
        }
    }
}