use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::Item;
use crate::modifier_gain::ModifierGain;
use crate::place_generator::generate_place;
use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemReport {
    item: Item,
    current_damage: HashMap<AttackType, u64>,
    effect_description: String,
    item_resource_costs: HashMap<ItemResourceType, u64>,
    current_item_resources: HashMap<ItemResourceType, u64>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteMoveCommandReport {
    item_report: Vec<ItemReport>,
    result: String,
}

pub fn execute_move_command(game: &mut Game, index: usize) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandReport> {
    if game.places.len() < index {
        return Err(ExecuteMoveCommandReport {
            item_report: Vec::new(),
            result: format!("Error: execute_move_command: Index {} is out of range of places, places is {} long.", index, game.places.len()),
        });
    }

    let mut current_damage = HashMap::new();
    let mut current_item_resources = game.item_resources.clone();
    let mut item_report = Vec::new();

    for item in &game.equipped_items {
        let mut item_resource_cost = HashMap::new();
        for modifier in &item.modifiers {
            for cost in &modifier.costs {
                match cost {
                    ModifierCost::FlatItemResource(item_resource_type, amount) => *item_resource_cost.entry(item_resource_type.clone()).or_insert(0) += amount
                }
            }
        }

        let are_all_costs_payable = item_resource_cost.iter()
            .all(|(item_resource_type, amount)|
                match current_item_resources.get(item_resource_type) {
                    None => false,
                    Some(stored_amount) => {
                        stored_amount >= amount
                    }
                }
            ).clone();
        if !are_all_costs_payable {
            item_report.push(ItemReport {
                item: item.clone(),
                current_damage: current_damage.clone(),
                effect_description: "Were not able to pay all the costs".to_string(),
                item_resource_costs: item_resource_cost.clone(),
                current_item_resources: current_item_resources.clone(),
            });
        } else {
            for (item_resource_cost_type, amount) in &item_resource_cost {
                current_item_resources.entry(item_resource_cost_type.clone()).and_modify(|current_amount| *current_amount -= amount);
            }

            for modifier in &item.modifiers {
                for gain in &modifier.gains {
                    match gain {
                        ModifierGain::FlatDamage(attack_type, amount) => *current_damage.entry(attack_type.clone()).or_insert(0) += amount,
                        ModifierGain::FlatItemResource(item_resource_type, amount) => *current_item_resources.entry(item_resource_type.clone()).or_insert(0) += amount,
                    }
                }
            }

            item_report.push(ItemReport {
                item: item.clone(),
                current_damage: current_damage.clone(),
                effect_description: "Costs paid and all gains executed.".to_string(),
                item_resource_costs: item_resource_cost.clone(),
                current_item_resources: current_item_resources.clone(),
            });

            game.item_resources = current_item_resources.clone();

            if let Some(rewards) = game.places.get(index)
                .expect("Error: execute_move_command: Could not find place even though it were within the index.")
                .claim_rewards(&current_damage) {
                for (treasure_type, amount) in rewards {
                    *game.treasure.entry(treasure_type).or_insert(0) += amount;
                }

                game.places[index] = generate_place(&game.place_generator_input);

                return Ok(ExecuteMoveCommandReport {
                    item_report,
                    result: "You won".to_string(),
                });
            }
        }
    }

    Err(ExecuteMoveCommandReport {
        item_report,
        result: "You did not deal enough damage to overcome the challenges in this place.".to_string(),
    })
}

#[cfg(test)]
mod tests_int {
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_new_game;
    use crate::item_resource::ItemResourceType;
    use crate::treasure_types::TreasureType;

    #[test]
    fn test_execute_move_command() {
        let mut game = generate_new_game();
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!(2, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
        assert_eq!(Some(&5), game.item_resources.get(&ItemResourceType::Mana));

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won".to_string(), result.result);
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&TreasureType::Gold), game.treasure.get(&TreasureType::Gold));
        assert_ne!(&0, game.treasure.get(&TreasureType::Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&ItemResourceType::Mana));
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = generate_new_game();

        let result = execute_move_command(&mut game, 11);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("Error: execute_move_command: Index 11 is out of range of places, places is 10 long.".to_string(), result.result);
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }

    #[test]
    fn test_execute_not_enough_damage() {
        let mut game = generate_new_game();
        game.equipped_items = Vec::new();

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }
}