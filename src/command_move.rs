use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::Item;
use crate::modifier_gain::ModifierGain;
use crate::place_generator::generate_place;
use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;
use crate::treasure_types::TreasureType;

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
    if game.places.len() < index { return report_place_does_not_exist(game, index); }

    let mut current_damage = HashMap::new();
    let mut current_item_resources = game.item_resources.clone();
    let mut item_report = Vec::new();

    for item in &game.equipped_items {
        let item_resource_cost = calculate_item_resource_cost(&item);
        let are_all_costs_payable = calculate_are_all_costs_payable(&mut current_item_resources, &item_resource_cost);
        if !are_all_costs_payable {
            update_item_report(&current_damage, &mut current_item_resources, &mut item_report, item, &item_resource_cost, "Were not able to pay all the costs");
        } else {
            update_cost_effect(&mut current_item_resources, &item_resource_cost);
            update_gain_effect(&mut current_damage, &mut current_item_resources, &item);
            update_item_report(&mut current_damage, &mut current_item_resources, &mut item_report, item, &item_resource_cost, "Costs paid and all gains executed.");

            game.item_resources = current_item_resources.clone();

            //If we can claim the reward.
            if let Some(rewards) = game.places.get(index)
                .expect("Error: execute_move_command: Could not find place even though it were within the index.")
                .claim_rewards(&current_damage) {
                return update_claim_place_effect(game, index, item_report, rewards);
            }
        }
    }

    Err(ExecuteMoveCommandReport {
        item_report,
        result: "You did not deal enough damage to overcome the challenges in this place.".to_string(),
    })
}

fn report_place_does_not_exist(game: &mut Game, index: usize) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandReport> {
    return Err(ExecuteMoveCommandReport {
        item_report: Vec::new(),
        result: format!("Error: execute_move_command: Index {} is out of range of places, places is {} long.", index, game.places.len()),
    });
}

fn update_claim_place_effect(game: &mut Game, index: usize, item_report: Vec<ItemReport>, rewards: HashMap<TreasureType, u64>) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandReport> {
    for (treasure_type, amount) in rewards {
        *game.treasure.entry(treasure_type).or_insert(0) += amount;
    }

    game.places[index] = generate_place(game);

    return Ok(ExecuteMoveCommandReport {
        item_report,
        result: "You won".to_string(),
    });
}

fn update_gain_effect(current_damage: &mut HashMap<AttackType, u64>, current_item_resources: &mut HashMap<ItemResourceType, u64>, item: &&Item) {
    for modifier in &item.modifiers {
        for gain in &modifier.gains {
            match gain {
                ModifierGain::FlatDamage(attack_type, amount) => *current_damage.entry(attack_type.clone()).or_insert(0) += amount,
                ModifierGain::FlatItemResource(item_resource_type, amount) => *current_item_resources.entry(item_resource_type.clone()).or_insert(0) += amount,
            }
        }
    }
}

fn update_cost_effect(current_item_resources: &mut HashMap<ItemResourceType, u64>, item_resource_cost: &HashMap<ItemResourceType, u64>) {
    for (item_resource_cost_type, amount) in item_resource_cost {
        current_item_resources.entry(item_resource_cost_type.clone()).and_modify(|current_amount| *current_amount -= amount);
    }
}

fn update_item_report(current_damage: &HashMap<AttackType, u64>, current_item_resources: &HashMap<ItemResourceType, u64>, item_report: &mut Vec<ItemReport>, item: &Item, item_resource_cost: &HashMap<ItemResourceType, u64>, effect_description: &str) {
    item_report.push(ItemReport {
        item: item.clone(),
        current_damage: current_damage.clone(),
        effect_description: effect_description.to_string(),
        item_resource_costs: item_resource_cost.clone(),
        current_item_resources: current_item_resources.clone(),
    });
}

fn calculate_are_all_costs_payable(current_item_resources: &HashMap<ItemResourceType, u64>, item_resource_cost: &HashMap<ItemResourceType, u64>) -> bool {
    let are_all_costs_payable = item_resource_cost.iter()
        .all(|(item_resource_type, amount)|
            match current_item_resources.get(item_resource_type) {
                None => false,
                Some(stored_amount) => {
                    stored_amount >= amount
                }
            }
        ).clone();
    are_all_costs_payable
}

fn calculate_item_resource_cost(item: &&Item) -> HashMap<ItemResourceType, u64> {
    let mut item_resource_cost = HashMap::new();
    for modifier in &item.modifiers {
        for cost in &modifier.costs {
            match cost {
                ModifierCost::FlatItemResource(item_resource_type, amount) => *item_resource_cost.entry(item_resource_type.clone()).or_insert(0) += amount
            }
        }
    }
    item_resource_cost
}

#[cfg(test)]
mod tests_int {
    use crate::attack_types::AttackType;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::item::Item;
    use crate::item_modifier::ItemModifier;
    use crate::item_resource::ItemResourceType;
    use crate::modifier_gain::ModifierGain;
    use crate::treasure_types::TreasureType;

    #[test]
    fn test_execute_move_command() {
        let mut game = generate_testing_game();
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
        assert_eq!(None, game.item_resources.get(&ItemResourceType::Mana));

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
        let mut game = generate_testing_game();

        let result = execute_move_command(&mut game, 11);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("Error: execute_move_command: Index 11 is out of range of places, places is 10 long.".to_string(), result.result);
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }

    #[test]
    fn test_execute_not_enough_damage() {
        let mut game = generate_testing_game();
        game.equipped_items = Vec::new();

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }

    #[test]
    fn test_execute_move_command_item_after_claim_does_not_activate() {
        let mut game = generate_testing_game();
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
        assert_eq!(None, game.item_resources.get(&ItemResourceType::Mana));

        let power_item = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: AttackType::get_all().iter()
                        .map(|attack_type| ModifierGain::FlatDamage(attack_type.clone(), 100))
                        .collect(),
                }
            ]
        };
        game.equipped_items.insert(0, power_item);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won".to_string(), result.result);
        assert_eq!(1, result.item_report.len()); //Only the first item got activated, because that were enough.
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&TreasureType::Gold), game.treasure.get(&TreasureType::Gold));
        assert_ne!(&0, game.treasure.get(&TreasureType::Gold).unwrap());
        assert_eq!(None, game.item_resources.get(&ItemResourceType::Mana));

        //Putting the power item at the end
        game.equipped_items.swap(0, 2);
        game.equipped_items.swap(0, 1);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won".to_string(), result.result);
        assert_eq!(3, result.item_report.len()); //Now all three have a report.
        assert_ne!(place, game.places[0]);
        assert!(place.reward.get(&TreasureType::Gold).unwrap() < game.treasure.get(&TreasureType::Gold).unwrap());
        assert_eq!(Some(&5), game.item_resources.get(&ItemResourceType::Mana));
    }
}