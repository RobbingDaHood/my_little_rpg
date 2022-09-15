use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::attack_types::AttackType;
use crate::Game;
use crate::item::{CraftingInfo, Item};
use crate::item_modifier::Modifier;
use crate::item_resource::Type;
use crate::modifier_cost::Cost;
use crate::modifier_gain::Gain;
use crate::place::Place;
use crate::place_generator::generate_place;
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ItemReport {
    item: Item,
    current_damage: HashMap<AttackType, u64>,
    current_resistance_reduction: HashMap<AttackType, u64>,
    treasure_bonus: HashMap<TreasureType, u16>,
    item_gain: u16,
    effect_description: String,
    item_resource_costs: Option<HashMap<Type, u64>>,
    current_item_resources: HashMap<Type, u64>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteMoveCommandReport {
    item_report: Vec<ItemReport>,
    result: String,
    new_place: Place,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteMoveCommandErrorReport {
    item_report: Vec<ItemReport>,
    result: String,
}

pub fn execute_move_command(game: &mut Game, index: usize) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandErrorReport> {
    if game.places.len() <= index { return report_place_does_not_exist(game, index); }

    game.game_statistics.moves_count += 1;

    let mut current_damage = HashMap::new();
    let mut current_resistance_reduction = HashMap::new();
    let mut treasure_bonus = HashMap::new();
    let mut item_gain = 1;
    let mut item_report = Vec::new();

    for item in &game.equipped_items {
        let item_resource_cost = match evaluate_item_costs(item, &current_damage, game, index) {
            Ok(costs) => costs,
            Err(message) => {
                item_report.push(ItemReport {
                    item: item.clone(),
                    current_damage: current_damage.clone(),
                    current_resistance_reduction: current_resistance_reduction.clone(),
                    treasure_bonus: treasure_bonus.clone(),
                    item_gain,
                    effect_description: message.to_string(),
                    item_resource_costs: None,
                    current_item_resources: game.item_resources.clone(),
                });
                continue;
            }
        };

        update_cost_effect(&mut game.item_resources, &item_resource_cost);
        update_gain_effect(&mut current_damage, &mut current_resistance_reduction, &mut treasure_bonus, &mut item_gain, &mut game.item_resources, item, game.places.get(index).unwrap());
        item_report.push(ItemReport {
            item: item.clone(),
            current_damage: current_damage.clone(),
            current_resistance_reduction: current_resistance_reduction.clone(),
            treasure_bonus: treasure_bonus.clone(),
            item_gain,
            effect_description: "Costs paid and all gains executed.".to_string(),
            item_resource_costs: Some(item_resource_cost),
            current_item_resources: game.item_resources.clone(),
        });

        //For the calculation of claiming the rewards we can merge the attack damage and flat resistance reduction into damage;
        let merged_damage_and_reduced_resistance = current_damage.keys().chain(current_resistance_reduction.keys())
            .map(|attack_type| {
                let current_damage_amount = current_damage.get(attack_type).unwrap_or(&0);
                let current_resistance_reduction_amount = current_resistance_reduction.get(attack_type).unwrap_or(&0);
                let merged_damage = current_damage_amount.checked_add(*current_resistance_reduction_amount).unwrap_or(u64::MAX);
                (attack_type.clone(), merged_damage)
            })
            .collect();

        //If we can claim the reward.
        if let Some(rewards) = game.places.get(index)
            .expect("Error: execute_move_command: Could not find place even though it were within the index.")
            .claim_rewards(&merged_damage_and_reduced_resistance) {
            game.game_statistics.wins += 1;
            game.game_statistics.wins_in_a_row += 1;
            game.game_statistics.loses_in_a_row = 0;

            let modified_rewards = rewards.iter()
                .map(|(treasure_type, treasure_amount)|
                    match treasure_bonus.get(treasure_type) {
                        None => (treasure_type.clone(), *treasure_amount),
                        Some(multiplier_as_percentage) => {
                            let multiplied_treasure_value = add_multiplier_to_base(*multiplier_as_percentage, *treasure_amount);
                            (treasure_type.clone(), multiplied_treasure_value)
                        }
                    }
                )
                .collect();

            for _i in 0..item_gain {
                game.inventory.push(Some(Item {
                    crafting_info: CraftingInfo {
                        possible_rolls: game.places[index].item_reward_possible_rolls.clone(),
                        places_count: game.places.len(),
                    },
                    modifiers: vec![
                        Modifier {
                            costs: Vec::new(),
                            gains: Vec::new(),
                        }
                    ],
                }));
            }

            return Ok(update_claim_place_effect(game, index, item_report, modified_rewards));
        }
    }

    game.game_statistics.loses += 1;
    game.game_statistics.loses_in_a_row += 1;
    game.game_statistics.wins_in_a_row = 0;

    Err(ExecuteMoveCommandErrorReport {
        item_report,
        result: "You did not deal enough damage to overcome the challenges in this place.".to_string(),
    })
}

//TODO Save stack of events, expose events and load events (Last part likely requires to be able to load files)

fn report_place_does_not_exist(game: &mut Game, index: usize) -> Result<ExecuteMoveCommandReport, ExecuteMoveCommandErrorReport> {
    Err(ExecuteMoveCommandErrorReport {
        item_report: Vec::new(),
        result: format!("Error: execute_move_command: Index {} is out of range of places, places is {} long.", index, game.places.len()),
    })
}

fn update_claim_place_effect(game: &mut Game, index: usize, item_report: Vec<ItemReport>, rewards: HashMap<TreasureType, u64>) -> ExecuteMoveCommandReport {
    for (treasure_type, amount) in rewards {
        *game.treasure.entry(treasure_type).or_insert(0) += amount;
    }

    game.places[index] = generate_place(game);

    ExecuteMoveCommandReport {
        item_report,
        result: "You won and got a new item in the inventory.".to_string(),
        new_place: game.places[index].clone(),
    }
}

fn update_gain_effect(current_damage: &mut HashMap<AttackType, u64>, current_resistance_reduction: &mut HashMap<AttackType, u64>, treasure_bonus: &mut HashMap<TreasureType, u16>, item_gain: &mut u16, current_item_resources: &mut HashMap<Type, u64>, item: &Item, place: &Place) {
    for modifier in &item.modifiers {
        for gain in &modifier.gains {
            match gain {
                Gain::FlatDamage(attack_type, amount) => *current_damage.entry(attack_type.clone()).or_insert(0) += amount,
                Gain::PercentageIncreaseDamage(attack_type, multiplier_as_percentage) => add_multiplier_to_attack_type_base(current_damage, attack_type, *multiplier_as_percentage),
                Gain::FlatItemResource(item_resource_type, amount) => *current_item_resources.entry(item_resource_type.clone()).or_insert(0) += amount,
                Gain::FlatResistanceReduction(attack_type, amount) => *current_resistance_reduction.entry(attack_type.clone()).or_insert(0) += amount,
                Gain::PercentageIncreaseResistanceReduction(attack_type, multiplier_as_percentage) => add_multiplier_to_attack_type_base(current_resistance_reduction, attack_type, *multiplier_as_percentage),
                Gain::FlatDamageAgainstHighestResistance(amount) => {
                    let attack_type_with_max_resistance = get_attack_type_with_max_amount(place);
                    *current_damage.entry(attack_type_with_max_resistance.clone()).or_insert(0) += amount;
                }
                Gain::PercentageIncreaseDamageAgainstHighestResistance(multiplier_as_percentage) => {
                    let attack_type_with_max_resistance = get_attack_type_with_max_amount(place);
                    add_multiplier_to_attack_type_base(current_damage, &attack_type_with_max_resistance, *multiplier_as_percentage);
                }
                Gain::FlatDamageAgainstLowestResistance(amount) => {
                    let attack_type_with_min_resistance = get_attack_type_with_min_amount(place);
                    *current_damage.entry(attack_type_with_min_resistance.clone()).or_insert(0) += amount;
                }
                Gain::PercentageIncreaseDamageAgainstLowestResistance(multiplier_as_percentage) => {
                    let attack_type_with_min_resistance = get_attack_type_with_min_amount(place);
                    add_multiplier_to_attack_type_base(current_damage, &attack_type_with_min_resistance, *multiplier_as_percentage);
                }
                Gain::PercentageIncreaseTreasure(treasure_type, amount) => *treasure_bonus.entry(treasure_type.clone()).or_insert(0) += amount,
                Gain::FlatIncreaseRewardedItems(amount) => *item_gain = item_gain.checked_add(*amount).unwrap_or(u16::MAX),
            }
        }
    }
}

fn get_attack_type_with_min_amount(place: &Place) -> AttackType {
    AttackType::order_map(&place.resistance).into_iter()
        .min_by(|(_, a_attack_amount), (_, b_attack_amount)| a_attack_amount.cmp(b_attack_amount))
        .map(|(attack_type, _)| attack_type)
        .unwrap()
}

fn get_attack_type_with_max_amount(place: &Place) -> AttackType {
    AttackType::order_map(&place.resistance).into_iter()
        .max_by(|(_, a_attack_amount), (_, b_attack_amount)| a_attack_amount.cmp(b_attack_amount))
        .map(|(attack_type, _)| attack_type)
        .unwrap()
}

fn add_multiplier_to_attack_type_base(attack_type_base: &mut HashMap<AttackType, u64>, attack_type: &AttackType, multiplier_as_percentage: u16) {
    match attack_type_base.get(attack_type) {
        None => {}
        Some(attack_value) => {
            let multiplied_attack_value = add_multiplier_to_base(multiplier_as_percentage, *attack_value);
            attack_type_base.insert(attack_type.clone(), multiplied_attack_value);
        }
    }
}

fn add_multiplier_to_base(multiplier_as_percentage: u16, base_value: u64) -> u64 {
    base_value
        .checked_mul(u64::from(multiplier_as_percentage))
        .unwrap_or(u64::MAX)
        .checked_div(100)
        .unwrap_or(1)
        .max(1)
        .checked_add(base_value)
        .unwrap_or(u64::MAX)
}

fn update_cost_effect(current_item_resources: &mut HashMap<Type, u64>, item_resource_cost: &HashMap<Type, u64>) {
    for (item_resource_cost_type, amount) in item_resource_cost {
        current_item_resources.entry(item_resource_cost_type.clone()).and_modify(|current_amount| *current_amount -= amount);
    }
}

fn calculate_are_all_costs_payable(current_item_resources: &HashMap<Type, u64>, item_resource_cost: &HashMap<Type, u64>) -> bool {
    let are_all_costs_payable = item_resource_cost.iter()
        .all(|(item_resource_type, amount)|
            match current_item_resources.get(item_resource_type) {
                None => false,
                Some(stored_amount) => {
                    stored_amount >= amount
                }
            }
        );
    are_all_costs_payable
}

fn evaluate_item_costs(item: &Item, current_damage: &HashMap<AttackType, u64>, game: &Game, index: usize) -> Result<HashMap<Type, u64>, String> {
    let mut item_resource_cost = HashMap::new();
    for modifier in &item.modifiers {
        for cost in &modifier.costs {
            match cost {
                Cost::FlatItemResource(item_resource_type, amount) => *item_resource_cost.entry(item_resource_type.clone()).or_insert(0) += amount,
                Cost::FlatMinItemResourceRequirement(item_resource_type, amount) => {
                    if *game.item_resources.get(item_resource_type).unwrap_or(&0) < *amount {
                        return Err(format!("Did not fulfill the FlatMinItemResourceRequirement of {} {:?}, only had {:?}.", amount, item_resource_type, game.item_resources.clone()));
                    }
                }
                Cost::FlatMaxItemResourceRequirement(item_resource_type, amount) => {
                    if *game.item_resources.get(item_resource_type).unwrap_or(&0) > *amount {
                        return Err(format!("Did not fulfill the FlatMaxItemResourceRequirement of {} {:?}, had {:?} and that is too much.", amount, item_resource_type, game.item_resources.clone()));
                    }
                }
                Cost::FlatMinAttackRequirement(attack_type, amount) => {
                    if current_damage.get(attack_type).unwrap_or(&0) < amount {
                        return Err(format!("Did not fulfill the FlatMinAttackRequirement of {} {:?} damage, only did {:?} damage.", amount, attack_type, current_damage));
                    }
                }
                Cost::FlatMaxAttackRequirement(attack_type, amount) => {
                    if current_damage.get(attack_type).unwrap_or(&0) > amount {
                        return Err(format!("Did not fulfill the FlatMaxAttackRequirement of {} {:?} damage, did {:?} damage and that is too much.", amount, attack_type, current_damage));
                    }
                }
                Cost::FlatSumMinAttackRequirement(amount) => {
                    if current_damage.values().sum::<u64>() < *amount {
                        return Err(format!("Did not fulfill the FlatSumMinAttackRequirement of {} damage, only did {:?} damage.", amount, current_damage));
                    }
                }
                Cost::FlatSumMaxAttackRequirement(amount) => {
                    if current_damage.values().sum::<u64>() > *amount {
                        return Err(format!("Did not fulfill the FlatSumMaxAttackRequirement of {} damage, did {:?} damage damage and that is too much.", amount, current_damage));
                    }
                }
                Cost::PlaceLimitedByIndexModulus(modulus, valid_values) => {
                    let modulus_value = index.rem_euclid(usize::from(*modulus));
                    if !valid_values.contains(&u8::try_from(modulus_value).unwrap()) {
                        return Err(format!("Did not fulfill the PlaceLimitedByIndexModulus: {} % {} = {} and that is not contained in {:?}.", index, modulus, modulus_value, valid_values));
                    }
                }
                Cost::FlatMinResistanceRequirement(attack_type, amount) => {
                    if game.places[index].resistance.get(attack_type).unwrap_or(&0) < amount {
                        return Err(format!("Did not fulfill the FlatMinResistanceRequirement of {} {:?} damage, place only has {:?} damage.", amount, attack_type, AttackType::order_map(&game.places[index].resistance)));
                    }
                }
                Cost::FlatMaxResistanceRequirement(attack_type, amount) => {
                    if game.places[index].resistance.get(attack_type).unwrap_or(&0) > amount {
                        return Err(format!("Did not fulfill the FlatMaxResistanceRequirement of {} {:?} damage, place has {:?} damage and that is too much.", amount, attack_type, AttackType::order_map(&game.places[index].resistance)));
                    }
                }
                Cost::FlatMinSumResistanceRequirement(amount) => {
                    let damage_sum = game.places[index].resistance.values().sum::<u64>();
                    if damage_sum < *amount {
                        return Err(format!("Did not fulfill the FlatMinSumResistanceRequirement of {} damage, place only has {:?} damage.", amount, damage_sum));
                    }
                }
                Cost::FlatMaxSumResistanceRequirement(amount) => {
                    let damage_sum = game.places[index].resistance.values().sum::<u64>();
                    if damage_sum > *amount {
                        return Err(format!("Did not fulfill the FlatMaxSumResistanceRequirement of {} damage, place has {:?} damage and that is too much.", amount, damage_sum));
                    }
                }
                Cost::MinWinsInARow(amount) => {
                    if game.game_statistics.wins_in_a_row < u64::from(*amount) {
                        return Err(format!("Did not fulfill the MinWinsInARow of {} win, only hase {:?} wins in a row.", amount, game.game_statistics.wins_in_a_row));
                    }
                }
                Cost::MaxWinsInARow(amount) => {
                    if game.game_statistics.wins_in_a_row > u64::from(*amount) {
                        return Err(format!("Did not fulfill the MaxWinsInARow of {} win, have {:?} wins in a row and that is too much.", amount, game.game_statistics.wins_in_a_row));
                    }
                }
            }
        }
    }

    if !calculate_are_all_costs_payable(&game.item_resources, &item_resource_cost) {
        return Err(format!("Were not able to pay all the costs. Had to pay {:?}, but only had {:?} available.", item_resource_cost, game.item_resources));
    }

    Ok(item_resource_cost)
}

#[cfg(test)]
mod tests_int {
    use crate::attack_types::AttackType;
    use crate::command_move::execute_move_command;
    use crate::Game;
    use crate::game_generator::generate_testing_game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::Modifier;
    use crate::item_resource::Type;
    use crate::modifier_cost::Cost;
    use crate::modifier_gain::Gain;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_move_command() {
        let mut game = generate_testing_game(Some([1; 16]));
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(None, game.item_resources.get(&Type::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(0, game.game_statistics.moves_count);
        assert_eq!(0, game.game_statistics.wins);
        assert_eq!(0, game.game_statistics.loses);
        assert_eq!(0, game.game_statistics.wins_in_a_row);
        assert_eq!(0, game.game_statistics.loses_in_a_row);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!(2, result.item_report.len());
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(Some(&5), game.item_resources.get(&Type::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(1, game.game_statistics.moves_count);
        assert_eq!(0, game.game_statistics.wins);
        assert_eq!(1, game.game_statistics.loses);
        assert_eq!(0, game.game_statistics.wins_in_a_row);
        assert_eq!(1, game.game_statistics.loses_in_a_row);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&Type::Mana));
        assert_eq!(10, game.inventory.len());
        assert_eq!(2, game.game_statistics.moves_count);
        assert_eq!(1, game.game_statistics.wins);
        assert_eq!(1, game.game_statistics.loses);
        assert_eq!(1, game.game_statistics.wins_in_a_row);
        assert_eq!(0, game.game_statistics.loses_in_a_row);
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = generate_testing_game(Some([1; 16]));

        let result = execute_move_command(&mut game, 11);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!("Error: execute_move_command: Index 11 is out of range of places, places is 10 long.".to_string(), result.result);
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&Gold));
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
        assert_eq!(None, game.treasure.get(&Gold));
    }

    #[test]
    fn test_execute_move_command_item_after_claim_does_not_activate() {
        let mut game = generate_testing_game(Some([1; 16]));
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(None, game.item_resources.get(&Type::Mana));

        let power_item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: AttackType::get_all().iter()
                        .map(|attack_type| Gain::FlatDamage(attack_type.clone(), 100))
                        .collect(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };
        game.equipped_items.insert(0, power_item);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!(1, result.item_report.len()); //Only the first item got activated, because that were enough.
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(None, game.item_resources.get(&Type::Mana));

        //Putting the power item at the end
        game.equipped_items.swap(0, 2);
        game.equipped_items.swap(0, 1);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!(3, result.item_report.len()); //Now all three have a report.
        assert_ne!(place, game.places[0]);
        assert!(place.reward.get(&Gold).unwrap() < game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&5), game.item_resources.get(&Type::Mana));
    }

    #[test]
    fn test_flat_min_attack_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::FlatMinAttackRequirement(AttackType::Physical, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamage(AttackType::Physical, 20)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);


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
                Modifier {
                    costs: vec![
                        Cost::FlatMaxAttackRequirement(AttackType::Physical, 1)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamage(AttackType::Physical, 3)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);


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
                Modifier {
                    costs: vec![
                        Cost::PlaceLimitedByIndexModulus(6, vec![1, 3, 4])
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);
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
                Modifier {
                    costs: vec![
                        Cost::FlatItemResource(Type::Mana, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = item_with_gains(&mut game);

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Were not able to pay all the costs. Had to pay {Mana: 20}, but only had {} available.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[2].effect_description);
    }

    fn item_with_gains(game: &mut Game) -> Item {
        Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatItemResource(Type::Mana, 20)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        }
    }

    #[test]
    fn test_flat_min_sum_attack_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::FlatSumMinAttackRequirement(20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamage(AttackType::Physical, 10)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);


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
                Modifier {
                    costs: vec![
                        Cost::FlatSumMaxAttackRequirement(20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamage(AttackType::Physical, 11)
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);


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
                Modifier {
                    costs: vec![
                        Cost::FlatMinItemResourceRequirement(Type::Mana, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = item_with_gains(&mut game);

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);


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
                Modifier {
                    costs: vec![
                        Cost::FlatMaxItemResourceRequirement(Type::Mana, 20)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = item_with_gains(&mut game);

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items.push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

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
                Modifier {
                    costs: vec![
                        Cost::FlatMinResistanceRequirement(AttackType::Fire, 18)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMinResistanceRequirement of 18 Fire damage, place only has [(Fire, 17), (Frost, 7), (Darkness, 6), (Nature, 70), (Corruption, 52), (Holy, 89)] damage.".to_string(), result.item_report[0].effect_description);

        let result = execute_move_command(&mut game, 7);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
    }

    #[test]
    fn test_flat_max_resistance_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::FlatMaxResistanceRequirement(AttackType::Fire, 17)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);

        let result = execute_move_command(&mut game, 7);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMaxResistanceRequirement of 17 Fire damage, place has [(Fire, 20), (Frost, 4), (Light, 11), (Darkness, 37), (Corruption, 80), (Holy, 59)] damage and that is too much.".to_string(), result.item_report[0].effect_description);
    }

    #[test]
    fn test_flat_min_sum_resistance_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::FlatMinSumResistanceRequirement(195)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute_move_command(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMinSumResistanceRequirement of 195 damage, place only has 194 damage.".to_string(), result.item_report[0].effect_description);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
    }

    #[test]
    fn test_flat_max_sum_resistance_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::FlatMaxSumResistanceRequirement(194)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute_move_command(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the FlatMaxSumResistanceRequirement of 194 damage, place has 241 damage and that is too much.".to_string(), result.item_report[0].effect_description);
    }

    #[test]
    fn test_min_win_row_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::MinWinsInARow(1)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.insert(0, first_item_cannot_pay);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the MinWinsInARow of 1 win, only hase 0 wins in a row.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!("Did not fulfill the MinWinsInARow of 1 win, only hase 0 wins in a row.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);

        let result = execute_move_command(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
    }

    #[test]
    fn test_max_win_row_requirement() {
        let mut game = generate_testing_game(Some([1; 16]));

        let first_item_cannot_pay = Item {
            modifiers: vec![
                Modifier {
                    costs: vec![
                        Cost::MaxWinsInARow(0)
                    ],
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.insert(0, first_item_cannot_pay);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);

        let result = execute_move_command(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Did not fulfill the MaxWinsInARow of 0 win, have 1 wins in a row and that is too much.".to_string(), result.item_report[0].effect_description);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[1].effect_description);
    }

    #[test]
    fn test_percentage_increase_damage() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamage(AttackType::Physical, 200),
                        Gain::PercentageIncreaseDamage(AttackType::Physical, 200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(600, *result.item_report[0].current_damage.get(&AttackType::Physical).unwrap());
    }

    #[test]
    fn test_flat_resistance_reduction_damage() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatResistanceReduction(AttackType::Physical, 200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(200, *result.item_report[0].current_resistance_reduction.get(&AttackType::Physical).unwrap());
    }

    #[test]
    fn test_percentage_increase_resistance_reduction() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatResistanceReduction(AttackType::Physical, 200),
                        Gain::PercentageIncreaseResistanceReduction(AttackType::Physical, 200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(600, *result.item_report[0].current_resistance_reduction.get(&AttackType::Physical).unwrap());
    }

    #[test]
    fn test_flat_damage_against_highest_resistance() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamageAgainstHighestResistance(200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(200, *result.item_report[0].current_damage.get(&AttackType::Holy).unwrap());
    }

    #[test]
    fn test_percentage_increase_damage_against_highest_resistance() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamageAgainstHighestResistance(200),
                        Gain::PercentageIncreaseDamageAgainstHighestResistance(200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(600, *result.item_report[0].current_damage.get(&AttackType::Holy).unwrap());
    }

    #[test]
    fn test_flat_damage_against_lowest_resistance() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamageAgainstLowestResistance(200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(200, *result.item_report[0].current_damage.get(&AttackType::Darkness).unwrap());
    }

    #[test]
    fn test_flat_damage_against_lowest_resistance_multiple() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamageAgainstLowestResistance(200),
                        Gain::FlatDamageAgainstLowestResistance(200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(400, *result.item_report[0].current_damage.get(&AttackType::Darkness).unwrap());
    }

    #[test]
    fn test_percentage_increase_damage_against_lowest_resistance() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: vec![
                        Gain::FlatDamageAgainstLowestResistance(200),
                        Gain::PercentageIncreaseDamageAgainstLowestResistance(200),
                    ],
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);


        let result = execute_move_command(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!("You did not deal enough damage to overcome the challenges in this place.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(600, *result.item_report[0].current_damage.get(&AttackType::Darkness).unwrap());
    }

    #[test]
    fn test_percentage_increase_treasure() {
        let mut game = generate_testing_game(Some([1; 16]));

        //Remove all costs of super item
        for modifier in &mut game.equipped_items[1].modifiers {
            modifier.costs = Vec::new();
        }

        game.equipped_items[1]
            .modifiers[0]
            .gains.push(Gain::PercentageIncreaseTreasure(Gold, 200));
        game.equipped_items[1]
            .modifiers[0]
            .gains.push(Gain::PercentageIncreaseTreasure(Gold, 300));

        let old_place = game.places[0].clone();

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(500, *result.item_report[1].treasure_bonus.get(&Gold).unwrap());

        assert_eq!(old_place.reward.get(&Gold).unwrap() * 6, *game.treasure.get(&Gold).unwrap());
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
    }

    #[test]
    fn test_increase_item_gain() {
        let mut game = generate_testing_game(Some([1; 16]));

        //Remove all costs of super item
        for modifier in &mut game.equipped_items[1].modifiers {
            modifier.costs = Vec::new();
        }

        game.equipped_items[1]
            .modifiers[0]
            .gains.push(Gain::FlatIncreaseRewardedItems(200));

        let old_inventory_count = game.inventory.len();

        let result = execute_move_command(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!("You won and got a new item in the inventory.".to_string(), result.result);
        assert_eq!("Costs paid and all gains executed.".to_string(), result.item_report[0].effect_description);
        assert_eq!(201, result.item_report[1].item_gain);

        assert_eq!(old_inventory_count + 201, game.inventory.len());
    }
}