use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    generator::place::new,
    my_little_rpg_errors::MyError,
    the_world::{
        damage_types::DamageType,
        item::{CraftingInfo, Item},
        item_modifier::Modifier,
        item_resource::Type,
        modifier_cost::Cost,
        modifier_gain::Gain,
        place::Place,
        treasure_types::TreasureType,
    },
    Game,
};

mod tests;

// TODO this file have too many responsibilities

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ItemReport {
    item: Item,
    current_damage: HashMap<DamageType, u64>,
    current_resistance_reduction: HashMap<DamageType, u64>,
    treasure_bonus: HashMap<TreasureType, u16>,
    item_gain: u16,
    effect_description: Box<str>,
    item_resource_costs: Option<HashMap<Type, u64>>,
    current_item_resources: HashMap<Type, u64>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteMoveCommandReport {
    item_report: Vec<ItemReport>,
    result: Box<str>,
    new_place: Place,
}

pub fn execute_move_command_json(
    game: &mut Game,
    index: usize,
) -> Value {
    match execute(game, index) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(
    game: &mut Game,
    index: usize,
) -> Result<ExecuteMoveCommandReport, MyError> {
    if game.places.len() <= index {
        return report_place_does_not_exist(game, index);
    }

    game.statistics.moves_count += 1;

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
                    effect_description: message.into(),
                    item_resource_costs: None,
                    current_item_resources: game.item_resources.clone(),
                });
                continue;
            }
        };

        update_cost_effect(&mut game.item_resources, &item_resource_cost);
        update_gain_effect(
            &mut current_damage,
            &mut current_resistance_reduction,
            &mut treasure_bonus,
            &mut item_gain,
            &mut game.item_resources,
            item,
            game.places.get(index).unwrap(),
        );
        item_report.push(ItemReport {
            item: item.clone(),
            current_damage: current_damage.clone(),
            current_resistance_reduction: current_resistance_reduction.clone(),
            treasure_bonus: treasure_bonus.clone(),
            item_gain,
            effect_description: "Costs paid and all gains executed.".into(),
            item_resource_costs: Some(item_resource_cost),
            current_item_resources: game.item_resources.clone(),
        });

        //For the calculation of claiming the rewards we can merge the attack damage and flat resistance reduction into damage;
        let merged_damage_and_reduced_resistance = current_damage
            .keys()
            .chain(current_resistance_reduction.keys())
            .map(|attack_type| {
                let current_damage_amount = current_damage.get(attack_type).unwrap_or(&0);
                let current_resistance_reduction_amount =
                    current_resistance_reduction.get(attack_type).unwrap_or(&0);
                let merged_damage = current_damage_amount
                    .checked_add(*current_resistance_reduction_amount)
                    .unwrap_or(u64::MAX);
                (attack_type, merged_damage)
            })
            .collect();

        //If we can claim the reward.
        if let Some(rewards) = game
            .places
            .get(index)
            .expect(
                "Error: execute_move_command: Could not find place even though it were within the \
                 index.",
            )
            .claim_rewards(&merged_damage_and_reduced_resistance)
        {
            game.statistics.wins += 1;
            game.statistics.wins_in_a_row += 1;
            game.statistics.loses_in_a_row = 0;

            let modified_rewards = rewards
                .into_iter()
                .map(|(treasure_type, treasure_amount)| {
                    match treasure_bonus.get(&treasure_type) {
                        None => (treasure_type, treasure_amount),
                        Some(multiplier_as_percentage) => {
                            let multiplied_treasure_value =
                                add_multiplier_to_base(*multiplier_as_percentage, treasure_amount);
                            (treasure_type, multiplied_treasure_value)
                        }
                    }
                })
                .collect();

            for _i in 0..item_gain {
                game.inventory.push(Some(Item {
                    crafting_info: CraftingInfo {
                        possible_rolls: game.places[index].item_reward_possible_rolls.clone(),
                        places_count: game.places.len(),
                    },
                    modifiers: vec![Modifier {
                        costs: Vec::new(),
                        gains: Vec::new(),
                    }],
                }));
            }

            return Ok(update_claim_place_effect(
                game,
                index,
                item_report,
                modified_rewards,
            ));
        }
    }

    game.statistics.loses += 1;
    game.statistics.loses_in_a_row += 1;
    game.statistics.wins_in_a_row = 0;

    Err(MyError::create_move_command_error(
        "You did not deal enough damage to overcome the challenges in this place.".to_string(),
        json!(item_report).to_string(),
    ))
}

//TODO Save stack of events, expose events and load events (Last part likely requires to be able to load files)

fn report_place_does_not_exist(
    game: &mut Game,
    index: usize,
) -> Result<ExecuteMoveCommandReport, MyError> {
    Err(MyError::create_move_command_error(
        format!(
            "Error: execute_move_command: Index {} is out of range of places, places is {} long.",
            index,
            game.places.len()
        ),
        "[]".to_string(),
    ))
}

fn update_claim_place_effect(
    game: &mut Game,
    index: usize,
    item_report: Vec<ItemReport>,
    rewards: HashMap<TreasureType, u64>,
) -> ExecuteMoveCommandReport {
    for (treasure_type, amount) in rewards {
        *game.treasure.entry(treasure_type).or_insert(0) += amount;
    }

    game.places[index] = new(game);

    ExecuteMoveCommandReport {
        item_report,
        result: "You won and got a new item in the inventory.".into(),
        new_place: game.places[index].clone(),
    }
}

fn update_gain_effect(
    current_damage: &mut HashMap<DamageType, u64>,
    current_resistance_reduction: &mut HashMap<DamageType, u64>,
    treasure_bonus: &mut HashMap<TreasureType, u16>,
    item_gain: &mut u16,
    current_item_resources: &mut HashMap<Type, u64>,
    item: &Item,
    place: &Place,
) {
    for modifier in &item.modifiers {
        for gain in &modifier.gains {
            match gain {
                Gain::FlatDamage(attack_type, amount) => {
                    *current_damage.entry(attack_type.clone()).or_insert(0) += amount;
                }
                Gain::PercentageIncreaseDamage(attack_type, multiplier_as_percentage) => {
                    add_multiplier_to_attack_type_base(
                        current_damage,
                        attack_type,
                        *multiplier_as_percentage,
                    );
                }
                Gain::FlatItemResource(item_resource_type, amount) => {
                    *current_item_resources
                        .entry(item_resource_type.clone())
                        .or_insert(0) += amount;
                }
                Gain::FlatResistanceReduction(attack_type, amount) => {
                    *current_resistance_reduction
                        .entry(attack_type.clone())
                        .or_insert(0) += amount;
                }
                Gain::PercentageIncreaseResistanceReduction(
                    attack_type,
                    multiplier_as_percentage,
                ) => {
                    add_multiplier_to_attack_type_base(
                        current_resistance_reduction,
                        attack_type,
                        *multiplier_as_percentage,
                    );
                }
                Gain::FlatDamageAgainstHighestResistance(amount) => {
                    let attack_type_with_max_resistance = get_attack_type_with_max_amount(place);
                    *current_damage
                        .entry(attack_type_with_max_resistance.clone())
                        .or_insert(0) += amount;
                }
                Gain::PercentageIncreaseDamageAgainstHighestResistance(
                    multiplier_as_percentage,
                ) => {
                    let attack_type_with_max_resistance = get_attack_type_with_max_amount(place);
                    add_multiplier_to_attack_type_base(
                        current_damage,
                        attack_type_with_max_resistance,
                        *multiplier_as_percentage,
                    );
                }
                Gain::FlatDamageAgainstLowestResistance(amount) => {
                    let attack_type_with_min_resistance = get_attack_type_with_min_amount(place);
                    *current_damage
                        .entry(attack_type_with_min_resistance.clone())
                        .or_insert(0) += amount;
                }
                Gain::PercentageIncreaseDamageAgainstLowestResistance(multiplier_as_percentage) => {
                    let attack_type_with_min_resistance = get_attack_type_with_min_amount(place);
                    add_multiplier_to_attack_type_base(
                        current_damage,
                        attack_type_with_min_resistance,
                        *multiplier_as_percentage,
                    );
                }
                Gain::PercentageIncreaseTreasure(treasure_type, amount) => {
                    *treasure_bonus.entry(treasure_type.clone()).or_insert(0) += amount;
                }
                Gain::FlatIncreaseRewardedItems(amount) => {
                    *item_gain = item_gain.checked_add(*amount).unwrap_or(u16::MAX);
                }
            }
        }
    }
}

fn get_attack_type_with_min_amount(place: &Place) -> &DamageType {
    place
        .resistance
        .iter()
        .min_by(|(_, a_attack_amount), (_, b_attack_amount)| a_attack_amount.cmp(b_attack_amount))
        .map(|(attack_type, _)| attack_type)
        .unwrap()
}

fn get_attack_type_with_max_amount(place: &Place) -> &DamageType {
    place
        .resistance
        .iter()
        .max_by(|(_, a_attack_amount), (_, b_attack_amount)| a_attack_amount.cmp(b_attack_amount))
        .map(|(attack_type, _)| attack_type)
        .unwrap()
}

fn add_multiplier_to_attack_type_base(
    attack_type_base: &mut HashMap<DamageType, u64>,
    attack_type: &DamageType,
    multiplier_as_percentage: u16,
) {
    attack_type_base
        .entry(attack_type.clone())
        .and_modify(|attack_value| {
            *attack_value = add_multiplier_to_base(multiplier_as_percentage, *attack_value);
        });
}

fn add_multiplier_to_base(
    multiplier_as_percentage: u16,
    base_value: u64,
) -> u64 {
    base_value
        .checked_mul(u64::from(multiplier_as_percentage))
        .unwrap_or(u64::MAX)
        .checked_div(100)
        .unwrap_or(1)
        .max(1)
        .checked_add(base_value)
        .unwrap_or(u64::MAX)
}

fn update_cost_effect(
    current_item_resources: &mut HashMap<Type, u64>,
    item_resource_cost: &HashMap<Type, u64>,
) {
    for (item_resource_cost_type, amount) in item_resource_cost {
        current_item_resources
            .entry(item_resource_cost_type.clone())
            .and_modify(|current_amount| *current_amount -= amount);
    }
}

fn calculate_are_all_costs_payable(
    current_item_resources: &HashMap<Type, u64>,
    item_resource_cost: &HashMap<Type, u64>,
) -> bool {
    let are_all_costs_payable = item_resource_cost
        .iter()
        .all(|(item_resource_type, amount)| {
            match current_item_resources.get(item_resource_type) {
                None => false,
                Some(stored_amount) => stored_amount >= amount,
            }
        });
    are_all_costs_payable
}

fn evaluate_item_costs(
    item: &Item,
    current_damage: &HashMap<DamageType, u64>,
    game: &Game,
    index: usize,
) -> Result<HashMap<Type, u64>, MyError> {
    let mut item_resource_cost = HashMap::new();
    for modifier in &item.modifiers {
        for cost in &modifier.costs {
            match cost {
                Cost::FlatItemResource(item_resource_type, amount) => {
                    *item_resource_cost
                        .entry(item_resource_type.clone())
                        .or_insert(0) += amount;
                }
                Cost::FlatMinItemResourceRequirement(item_resource_type, amount) => {
                    let resource_amount = game.item_resources.get(item_resource_type).unwrap_or(&0);
                    if resource_amount < amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMinItemResourceRequirement of {amount} \
                             {item_resource_type:?}, only had {resource_amount}."
                        )));
                    }
                }
                Cost::FlatMaxItemResourceRequirement(item_resource_type, amount) => {
                    let resource_amount = game.item_resources.get(item_resource_type).unwrap_or(&0);
                    if resource_amount > amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMaxItemResourceRequirement of {amount} \
                             {item_resource_type:?}, had {resource_amount:?} and that is too much."
                        )));
                    }
                }
                Cost::FlatMinAttackRequirement(attack_type, amount) => {
                    if current_damage.get(attack_type).unwrap_or(&0) < amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMinAttackRequirement of {amount} \
                             {attack_type:?} damage, only did {current_damage:?} damage."
                        )));
                    }
                }
                Cost::FlatMaxAttackRequirement(attack_type, amount) => {
                    if current_damage.get(attack_type).unwrap_or(&0) > amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMaxAttackRequirement of {amount} \
                             {attack_type:?} damage, did {current_damage:?} damage and that is \
                             too much."
                        )));
                    }
                }
                Cost::FlatSumMinAttackRequirement(amount) => {
                    if current_damage.values().sum::<u64>() < *amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatSumMinAttackRequirement of {amount} damage, \
                             only did {current_damage:?} damage."
                        )));
                    }
                }
                Cost::FlatSumMaxAttackRequirement(amount) => {
                    if current_damage.values().sum::<u64>() > *amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatSumMaxAttackRequirement of {amount} damage, \
                             did {current_damage:?} damage damage and that is too much."
                        )));
                    }
                }
                Cost::PlaceLimitedByIndexModulus(modulus, valid_values) => {
                    let modulus_value = index.rem_euclid(usize::from(*modulus));
                    if !valid_values.contains(&u8::try_from(modulus_value).unwrap()) {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the PlaceLimitedByIndexModulus: {index} % {modulus} \
                             = {modulus_value} and that is not contained in {valid_values:?}."
                        )));
                    }
                }
                Cost::FlatMinResistanceRequirement(attack_type, amount) => {
                    let resistance_amount_place =
                        game.places[index].resistance.get(attack_type).unwrap_or(&0);
                    if resistance_amount_place < amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMinResistanceRequirement of {amount} \
                             {attack_type:?} damage, place only has {resistance_amount_place:?} \
                             damage."
                        )));
                    }
                }
                Cost::FlatMaxResistanceRequirement(attack_type, amount) => {
                    let resistance_amount_place =
                        game.places[index].resistance.get(attack_type).unwrap_or(&0);
                    if resistance_amount_place > amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMaxResistanceRequirement of {amount} \
                             {attack_type:?} damage, place has {resistance_amount_place:?} damage \
                             and that is too much."
                        )));
                    }
                }
                Cost::FlatMinSumResistanceRequirement(amount) => {
                    let damage_sum = game.places[index].resistance.values().sum::<u64>();
                    if damage_sum < *amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMinSumResistanceRequirement of {amount} \
                             damage, place only has {damage_sum:?} damage."
                        )));
                    }
                }
                Cost::FlatMaxSumResistanceRequirement(amount) => {
                    let damage_sum = game.places[index].resistance.values().sum::<u64>();
                    if damage_sum > *amount {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the FlatMaxSumResistanceRequirement of {amount} \
                             damage, place has {damage_sum:?} damage and that is too much."
                        )));
                    }
                }
                Cost::MinWinsInARow(amount) => {
                    if game.statistics.wins_in_a_row < u64::from(*amount) {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the MinWinsInARow of {} win, only hase {:?} wins in \
                             a row.",
                            amount, game.statistics.wins_in_a_row
                        )));
                    }
                }
                Cost::MaxWinsInARow(amount) => {
                    if game.statistics.wins_in_a_row > u64::from(*amount) {
                        return Err(MyError::create_execute_command_error(format!(
                            "Did not fulfill the MaxWinsInARow of {} win, have {:?} wins in a row \
                             and that is too much.",
                            amount, game.statistics.wins_in_a_row
                        )));
                    }
                }
            }
        }
    }

    if !calculate_are_all_costs_payable(&game.item_resources, &item_resource_cost) {
        return Err(MyError::create_execute_command_error(format!(
            "Were not able to pay all the costs. Had to pay {:?}, but only had {:?} available.",
            item_resource_cost, game.item_resources
        )));
    }

    Ok(item_resource_cost)
}
