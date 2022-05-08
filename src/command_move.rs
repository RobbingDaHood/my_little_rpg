use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::{CraftingInfo, Item};
use crate::modifier_gain::ModifierGain;
use crate::place_generator::generate_place;
use serde::{Deserialize, Serialize};
use crate::item_modifier::ItemModifier;
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;
use crate::place::Place;
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemReport {
    item: Item,
    current_damage: HashMap<AttackType, u64>,
    effect_description: String,
    item_resource_costs: Option<HashMap<ItemResourceType, u64>>,
    current_item_resources: HashMap<ItemResourceType, u64>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteMoveCommandReport {
    item_report: Vec<ItemReport>,
    result: String,
    new_place: Place,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteMoveCommandErrorReport {
    item_report: Vec<ItemReport>,
    result: String,
}

pub fn execute_move_command(game: &mut Game, index: usize) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandErrorReport> {
    if game.places.len() <= index { return report_place_does_not_exist(game, index); }

    game.game_statistics.moves_count += 1;

    let mut current_damage = HashMap::new();
    let mut item_report = Vec::new();

    for item in &game.equipped_items {
        let item_resource_cost = match evaluate_item_costs(&item, &current_damage, &game, index) {
            Ok(costs) => costs,
            Err((message, item_resource_cost)) => {
                update_item_report(&current_damage, &game.item_resources, &mut item_report, item, &item_resource_cost, message.as_str());
                continue;
            }
        };

        update_cost_effect(&mut game.item_resources, &item_resource_cost);
        update_gain_effect(&mut current_damage, &mut game.item_resources, &item);
        update_item_report(&current_damage, &game.item_resources, &mut item_report, item, &Some(item_resource_cost), "Costs paid and all gains executed.");

        //If we can claim the reward.
        if let Some(rewards) = game.places.get(index)
            .expect("Error: execute_move_command: Could not find place even though it were within the index.")
            .claim_rewards(&current_damage) {
            return update_claim_place_effect(game, index, item_report, rewards);
        }
    }

    Err(ExecuteMoveCommandErrorReport {
        item_report,
        result: "You did not deal enough damage to overcome the challenges in this place.".to_string(),
    })
}

fn report_place_does_not_exist(game: &mut Game, index: usize) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandErrorReport> {
    return Err(ExecuteMoveCommandErrorReport {
        item_report: Vec::new(),
        result: format!("Error: execute_move_command: Index {} is out of range of places, places is {} long.", index, game.places.len()),
    });
}

fn update_claim_place_effect(game: &mut Game, index: usize, item_report: Vec<ItemReport>, rewards: HashMap<TreasureType, u64>) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandErrorReport> {
    for (treasure_type, amount) in rewards {
        *game.treasure.entry(treasure_type).or_insert(0) += amount;
    }

    game.inventory.push(Some(Item {
        crafting_info: CraftingInfo {
            possible_rolls: game.places[index].item_reward_possible_rolls.clone(),
        },
        modifiers: vec![
            ItemModifier {
                costs: Vec::new(),
                gains: Vec::new(),
            }
        ],
    }));

    game.places[index] = generate_place(game);

    return Ok(ExecuteMoveCommandReport {
        item_report,
        result: "You won and got a new item in the inventory.".to_string(),
        new_place: game.places[index].clone(),
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

fn update_item_report(current_damage: &HashMap<AttackType, u64>, current_item_resources: &HashMap<ItemResourceType, u64>, item_report: &mut Vec<ItemReport>, item: &Item, item_resource_cost: &Option<HashMap<ItemResourceType, u64>>, effect_description: &str) {
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

fn evaluate_item_costs(item: &&Item, current_damage: &HashMap<AttackType, u64>, game: &Game, index: usize) -> Result<HashMap<ItemResourceType, u64>, (String, Option<HashMap<ItemResourceType, u64>>)> {
    let mut item_resource_cost = HashMap::new();
    for modifier in &item.modifiers {
        for cost in &modifier.costs {
            match cost {
                ModifierCost::FlatItemResource(item_resource_type, amount) => *item_resource_cost.entry(item_resource_type.clone()).or_insert(0) += amount,
                ModifierCost::FlatMinItemResourceRequirement(item_resource_type, amount) => {
                    if *game.item_resources.get(item_resource_type).unwrap_or(&0) < *amount {
                        return Err((format!("Did not fulfill the FlatMinItemResourceRequirement of {} {:?}, only had {:?}.", amount, item_resource_type, game.item_resources.clone()), None));
                    } else {}
                }
                ModifierCost::FlatMaxItemResourceRequirement(item_resource_type, amount) => {
                    if *game.item_resources.get(item_resource_type).unwrap_or(&0) > *amount {
                        return Err((format!("Did not fulfill the FlatMaxItemResourceRequirement of {} {:?}, had {:?} and that is too much.", amount, item_resource_type, game.item_resources.clone()), None));
                    } else {}
                }
                ModifierCost::FlatMinAttackRequirement(attack_type, amount) => {
                    if current_damage.get(attack_type).unwrap_or(&0) < amount {
                        return Err((format!("Did not fulfill the FlatMinAttackRequirement of {} {:?} damage, only did {:?} damage.", amount, attack_type, current_damage), None));
                    } else {}
                }
                ModifierCost::FlatMaxAttackRequirement(attack_type, amount) => {
                    if current_damage.get(attack_type).unwrap_or(&0) > amount {
                        return Err((format!("Did not fulfill the FlatMaxAttackRequirement of {} {:?} damage, did {:?} damage and that is too much.", amount, attack_type, current_damage), None));
                    } else {}
                }
                ModifierCost::FlatSumMinAttackRequirement(amount) => {
                    if current_damage.values().sum::<u64>() < *amount {
                        return Err((format!("Did not fulfill the FlatSumMinAttackRequirement of {} damage, only did {:?} damage.", amount, current_damage), None));
                    } else {}
                }
                ModifierCost::FlatSumMaxAttackRequirement(amount) => {
                    if current_damage.values().sum::<u64>() > *amount {
                        return Err((format!("Did not fulfill the FlatSumMaxAttackRequirement of {} damage, did {:?} damage damage and that is too much.", amount, current_damage), None));
                    } else {}
                }
                ModifierCost::PlaceLimitedByIndexModulus(modulus, valid_values) => {
                    let modulus_value = index.rem_euclid(usize::from(*modulus));
                    if !valid_values.contains(&u8::try_from(modulus_value).unwrap()) {
                        return Err((format!("Did not fulfill the PlaceLimitedByIndexModulus: {} % {} = {} and that is not contained in {:?}.", index, modulus, modulus_value, valid_values), None));
                    } else {}
                }
                ModifierCost::FlatMinResistanceRequirement(attack_type, amount) => {
                    let resustance = game.places[index].resistance.get(attack_type).unwrap_or(&0);
                    if resustance < amount {
                        return Err((format!("Did not fulfill the FlatMinResistanceRequirement of {} {:?} damage, place only has {:?} damage.", amount, attack_type, AttackType::order_map(&game.places[index].resistance)), None));
                    } else {}
                }
            }
        }
    }

    if !calculate_are_all_costs_payable(&game.item_resources, &item_resource_cost) {
        return Err((format!("Were not able to pay all the costs. Had to pay {:?}, but only had {:?} available.", item_resource_cost, game.item_resources), None));
    }

    Ok(item_resource_cost)
}

#[cfg(test)]
mod tests_int {
    use crate::attack_types::AttackType;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::ItemModifier;
    use crate::item_resource::ItemResourceType;
    use crate::modifier_cost::ModifierCost;
    use crate::modifier_gain::ModifierGain;
    use crate::treasure_types::TreasureType;

    #[test]
    fn test_execute_move_command() {
        let mut game = generate_testing_game(Some([1; 16]));
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
        assert_eq!(None, game.item_resources.get(&ItemResourceType::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(0, game.game_statistics.moves_count);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!(2, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
        assert_eq!(Some(&5), game.item_resources.get(&ItemResourceType::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(1, game.game_statistics.moves_count);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&TreasureType::Gold), game.treasure.get(&TreasureType::Gold));
        assert_ne!(&0, game.treasure.get(&TreasureType::Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&ItemResourceType::Mana));
        assert_eq!(10, game.inventory.len());
        assert_eq!(2, game.game_statistics.moves_count);
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = generate_testing_game(Some([1; 16]));

        let result = execute_move_command(&mut game, 11);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("Error: execute_move_command: Index 11 is out of range of places, places is 10 long.".to_string(), result.result);
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }

    #[test]
    fn test_execute_not_enough_damage() {
        let mut game = generate_testing_game(Some([1; 16]));
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
        let mut game = generate_testing_game(Some([1; 16]));
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
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };
        game.equipped_items.insert(0, power_item);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
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
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!(3, result.item_report.len()); //Now all three have a report.
        assert_ne!(place, game.places[0]);
        assert!(place.reward.get(&TreasureType::Gold).unwrap() < game.treasure.get(&TreasureType::Gold).unwrap());
        assert_eq!(Some(&5), game.item_resources.get(&ItemResourceType::Mana));
    }

    #[test]
    fn test_flat_min_attack_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatMinAttackRequirement(AttackType::Physical, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatDamage(AttackType::Physical, 20)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMinAttackRequirement of 20 Physical damage, only did {} damage.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[2].effect_description);
    }

    #[test]
    fn test_flat_max_attack_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatMaxAttackRequirement(AttackType::Physical, 1)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatDamage(AttackType::Physical, 3)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Did not fulfill the FlatMaxAttackRequirement of 1 Physical damage, did {Physical: 3} damage and that is too much.".to_string(), result.item_report[2].effect_description);
    }

    #[test]
    fn test_place_limited_by_index_modulus_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::PlaceLimitedByIndexModulus(6, vec![1, 3, 4])
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        assert_eq!("Did not fulfill the PlaceLimitedByIndexModulus: 0 % 6 = 0 and that is not contained in [1, 3, 4].".to_string(), execute_move_command(&mut game, 0).unwrap_err().item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), execute_move_command(&mut game, 1).unwrap_err().item_report[0].effect_description);
        assert_eq!("Did not fulfill the PlaceLimitedByIndexModulus: 2 % 6 = 2 and that is not contained in [1, 3, 4].".to_string(), execute_move_command(&mut game, 2).unwrap_err().item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), execute_move_command(&mut game, 3).unwrap_err().item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), execute_move_command(&mut game, 4).unwrap_err().item_report[0].effect_description);
        assert_eq!("Did not fulfill the PlaceLimitedByIndexModulus: 5 % 6 = 5 and that is not contained in [1, 3, 4].".to_string(), execute_move_command(&mut game, 5).unwrap_err().item_report[0].effect_description);
        assert_eq!("Did not fulfill the PlaceLimitedByIndexModulus: 6 % 6 = 0 and that is not contained in [1, 3, 4].".to_string(), execute_move_command(&mut game, 6).unwrap_err().item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), execute_move_command(&mut game, 7).unwrap_err().item_report[0].effect_description);
        assert_eq!("Did not fulfill the PlaceLimitedByIndexModulus: 8 % 6 = 2 and that is not contained in [1, 3, 4].".to_string(), execute_move_command(&mut game, 8).unwrap_err().item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), execute_move_command(&mut game, 9).unwrap_err().item_report[0].effect_description);
    }

    #[test]
    fn test_flat_item_resource() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatItemResource(ItemResourceType::Mana, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatItemResource(ItemResourceType::Mana, 20)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Were not able to pay all the costs. Had to pay {Mana: 20}, but only had {} available.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[2].effect_description);
    }

    #[test]
    fn test_flat_min_sum_attack_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatSumMinAttackRequirement(20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatDamage(AttackType::Physical, 10)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatSumMinAttackRequirement of 20 damage, only did {} damage.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Did not fulfill the FlatSumMinAttackRequirement of 20 damage, only did {Physical: 10} damage.".to_string(), result.item_report[2].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[3].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[4].effect_description);
    }

    #[test]
    fn test_flat_max_sum_attack_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatSumMaxAttackRequirement(20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatDamage(AttackType::Physical, 11)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[2].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[3].effect_description);
        assert_eq!("Did not fulfill the FlatSumMaxAttackRequirement of 20 damage, did {Physical: 22} damage damage and that is too much.".to_string(), result.item_report[4].effect_description);
    }

    //TODO add tests that check several execute_move_command in a row, for the item resource accumulation.

    #[test]
    fn test_flat_min_sum_item_resource_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatMinItemResourceRequirement(ItemResourceType::Mana, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatItemResource(ItemResourceType::Mana, 20)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMinItemResourceRequirement of 20 Mana, only had {}.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[2].effect_description);
    }

    #[test]
    fn test_flat_max_sum_item_resource_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatMaxItemResourceRequirement(ItemResourceType::Mana, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: vec![
                        ModifierGain::FlatItemResource(ItemResourceType::Mana, 20)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[2].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[3].effect_description);
        assert_eq!("Did not fulfill the FlatMaxItemResourceRequirement of 20 Mana, had {Mana: 40} and that is too much.".to_string(), result.item_report[4].effect_description);
    }

    #[test]
    fn test_flat_min_resistance_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                ItemModifier {
                    costs: vec![
                        ModifierCost::FlatMinResistanceRequirement(AttackType::Fire, 12)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMinResistanceRequirement of 12 Fire damage, place only has [(Frost, 7), (Lightning, 11), (Light, 6), (Darkness, 6), (Nature, 35), (Corruption, 28)] damage.".to_string(), result.item_report[0].effect_description);

        let result = execute_move_command(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
    }
}