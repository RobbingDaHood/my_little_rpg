use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::Add;

use rand::Rng;
use rand_pcg::Lcg64Xsh32;

use crate::attack_types::AttackType;
use crate::game::get_random_attack_type_from_unlocked;
use crate::item::CraftingInfo;
use crate::item_modifier::Modifier;
use crate::item_resource::Type;
use crate::modifier_cost::Cost;
use crate::modifier_gain::Gain::{FlatDamageAgainstHighestResistance, FlatDamageAgainstLowestResistance, FlatIncreaseRewardedItems, FlatItemResource, FlatResistanceReduction, PercentageIncreaseDamage, PercentageIncreaseDamageAgainstHighestResistance, PercentageIncreaseDamageAgainstLowestResistance, PercentageIncreaseResistanceReduction, PercentageIncreaseTreasure};
use crate::modifier_gain::Gain;
use crate::modifier_gain::Gain::FlatDamage;

pub fn execute_craft(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo) -> Modifier {
    let minimum_elements = min(crafting_info.possible_rolls.min_resistance.len(), crafting_info.possible_rolls.min_simultaneous_resistances as usize);
    let maximum_elements = min(crafting_info.possible_rolls.max_resistance.len(), crafting_info.possible_rolls.max_simultaneous_resistances as usize);

    let (modifier_costs, cost) = execute_craft_roll_modifier_costs(random_generator_state, crafting_info);

    let modifier_gain = execute_craft_roll_modifier_benefits(random_generator_state, crafting_info, cost, minimum_elements, maximum_elements);

    Modifier {
        costs: modifier_costs,
        gains: modifier_gain,
    }
}

fn execute_craft_roll_modifier_costs(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo) -> (Vec<Cost>, u64) {
    let mut modifier_costs = Vec::new();
    let mut accumulated_cost = 0;
    let max_cost = crafting_info.possible_rolls.max_resistance.values().sum::<u64>() / u64::from(crafting_info.possible_rolls.max_simultaneous_resistances);

    //TODO unblocked damage will apply unique effect

    let number_of_costs = random_generator_state.gen_range(0..crafting_info.possible_rolls.max_simultaneous_resistances.add(1));

    for _i in 0..number_of_costs {
        if accumulated_cost < max_cost {
            accumulated_cost += match random_generator_state.gen_range(0..14) {
                0 => add_flat_min_attack(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                1 => add_flat_max_attack(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                2 => add_place_limited_by_index_modulus(random_generator_state, crafting_info, &mut modifier_costs),
                3 => add_flat_sum_min_attach(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                4 => add_flat_sum_max_attack(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                5 => add_flat_min_item_resource(random_generator_state, &mut modifier_costs, accumulated_cost, max_cost),
                6 => add_flat_max_item_resource(random_generator_state, &mut modifier_costs, accumulated_cost, max_cost),
                7 => add_min_resistance(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                8 => add_flat_max_resistance(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                9 => add_flat_min_sum_resistance(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                10 => add_flat_max_sum_resistance(random_generator_state, crafting_info, &mut modifier_costs, accumulated_cost, max_cost),
                11 => add_min_wins_in_row(random_generator_state, crafting_info, &mut modifier_costs),
                12 => add_max_wins_in_row(random_generator_state, crafting_info, &mut modifier_costs),
                _ => add_flat_item_resource(random_generator_state, &mut modifier_costs, accumulated_cost, max_cost)
            }
        }
    }

    (modifier_costs, accumulated_cost)
}

fn add_flat_min_attack(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(random_generator_state, &crafting_info.possible_rolls.min_resistance);

    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatMinAttackRequirement(attack_type, value));
    value
}

fn add_flat_max_attack(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(random_generator_state, &crafting_info.possible_rolls.min_resistance);

    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatMaxAttackRequirement(attack_type, value));
    maximum_value - value
}

fn add_place_limited_by_index_modulus(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>) -> u64 {
    let max_modulus = min(usize::from(u8::MAX), crafting_info.places_count);
    let max_modulus = max(3, max_modulus);
    let modulus = u8::try_from(random_generator_state.gen_range(2..max_modulus)).unwrap();
    let number_of_valid_values = random_generator_state.gen_range(1..modulus);

    let valid_numbers = (0..number_of_valid_values).into_iter()
        .map(|_| random_generator_state.gen_range(0..modulus))
        .collect::<HashSet<u8>>().into_iter()
        .collect();

    modifier_costs.push(Cost::PlaceLimitedByIndexModulus(modulus, valid_numbers));

    (crafting_info.places_count * ((modulus / number_of_valid_values) as usize)) as u64
}

fn add_flat_sum_min_attach(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatSumMinAttackRequirement(value));
    value
}

fn add_flat_sum_max_attack(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatSumMaxAttackRequirement(value));
    maximum_value - value
}

fn add_flat_min_item_resource(random_generator_state: &mut Lcg64Xsh32, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let cost = random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
    modifier_costs.push(Cost::FlatMinItemResourceRequirement(Type::Mana, cost));
    cost
}

fn add_flat_max_item_resource(random_generator_state: &mut Lcg64Xsh32, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let cost = random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
    modifier_costs.push(Cost::FlatMaxItemResourceRequirement(Type::Mana, cost));
    (max_cost - accumulated_cost) - cost //TODO Better cost
}

fn add_min_resistance(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(random_generator_state, &crafting_info.possible_rolls.min_resistance);

    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatMinResistanceRequirement(attack_type, value));
    value
}

fn add_flat_max_resistance(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(random_generator_state, &crafting_info.possible_rolls.min_resistance);

    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatMaxResistanceRequirement(attack_type, value));
    maximum_value - value
}

fn add_flat_min_sum_resistance(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatMinSumResistanceRequirement(value));
    value
}

fn add_flat_max_sum_resistance(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
    let value = min(max_cost - accumulated_cost, random_generator_state.gen_range(minimum_value..=maximum_value));

    modifier_costs.push(Cost::FlatMaxSumResistanceRequirement(value));
    maximum_value - value
}

fn add_min_wins_in_row(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>) -> u64 {
    let value = u8::try_from(min(usize::from(u8::MAX), crafting_info.places_count * 2)).unwrap();
    let value = max(2, value);
    let value = random_generator_state.gen_range(1..value);

    modifier_costs.push(Cost::MinWinsInARow(value));
    u64::from(value)
}

fn add_max_wins_in_row(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, modifier_costs: &mut Vec<Cost>) -> u64 {
    let max_value = u8::try_from(min(usize::from(u8::MAX), crafting_info.places_count * 2)).unwrap();
    let value = max(1, max_value);
    let value = random_generator_state.gen_range(0..value);

    modifier_costs.push(Cost::MaxWinsInARow(value));
    u64::from(max_value - value)
}

fn add_flat_item_resource(random_generator_state: &mut Lcg64Xsh32, modifier_costs: &mut Vec<Cost>, accumulated_cost: u64, max_cost: u64) -> u64 {
    let cost = random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
    modifier_costs.push(Cost::FlatItemResource(Type::Mana, cost));
    cost
}

fn execute_craft_roll_modifier_benefits(random_generator_state: &mut Lcg64Xsh32, crafting_info: &CraftingInfo, cost: u64, minimum_elements: usize, maximum_elements: usize) -> Vec<Gain> {
    let attack_types = AttackType::get_all().iter()
        .filter(|attack_type| crafting_info.possible_rolls.min_resistance.contains_key(attack_type))
        .cloned()
        .collect::<Vec<AttackType>>();

    let mut leftover_cost = cost;

    let all_modifier_gain_options = Gain::get_all_given_attack_types(attack_types);
    let mut modifier_gain = Vec::new();
    for i in minimum_elements..=maximum_elements {
        let cost_bonus = if i == maximum_elements {
            leftover_cost
        } else {
            random_generator_state.gen_range(1..=leftover_cost)
        };
        leftover_cost -= cost_bonus;

        let gain_seize = all_modifier_gain_options.len();
        let modifier_index = random_generator_state.gen_range(0..gain_seize);
        modifier_gain.push(
            match &all_modifier_gain_options[modifier_index] { //TODO do the same with costs.
                FlatDamage(attack_type, _) => {
                    let min_damage = *crafting_info.possible_rolls.min_resistance.get(attack_type).unwrap_or(&0);
                    let max_damage = *crafting_info.possible_rolls.max_resistance.get(attack_type).unwrap_or(&1);
                    let damage = random_generator_state.gen_range(min_damage..=max_damage);
                    let damage = damage / 2;
                    let damage = max(1, damage);
                    let damage = damage + cost_bonus * 2;

                    FlatDamage(attack_type.clone(), damage)
                }
                PercentageIncreaseDamage(attack_type, _) => {
                    PercentageIncreaseDamage(attack_type.clone(), u16::try_from(cost_bonus).unwrap_or(u16::MAX).max(1))
                }
                FlatItemResource(item_resource_type, _) => {
                    FlatItemResource(item_resource_type.clone(), max(1, cost_bonus * 2))
                }
                FlatResistanceReduction(attack_type, _) => {
                    let min_damage = *crafting_info.possible_rolls.min_resistance.get(attack_type).unwrap_or(&0);
                    let max_damage = *crafting_info.possible_rolls.max_resistance.get(attack_type).unwrap_or(&1);
                    let damage = random_generator_state.gen_range(min_damage..=max_damage);
                    let damage = damage / 2;
                    let damage = max(1, damage);
                    let damage = damage + cost_bonus * 2;

                    FlatResistanceReduction(attack_type.clone(), damage)
                }
                PercentageIncreaseResistanceReduction(attack_type, _) => {
                    PercentageIncreaseResistanceReduction(attack_type.clone(), u16::try_from(cost_bonus).unwrap_or(u16::MAX).max(1))
                }
                FlatDamageAgainstHighestResistance(_) => {
                    FlatDamageAgainstHighestResistance(cost_bonus.checked_div(2).unwrap_or(1).max(1))
                }
                PercentageIncreaseDamageAgainstHighestResistance(_) => {
                    PercentageIncreaseDamageAgainstHighestResistance(u16::try_from(cost_bonus.checked_div(2).unwrap_or(1).max(1)).unwrap_or(u16::MAX))
                }
                FlatDamageAgainstLowestResistance(_) => {
                    FlatDamageAgainstLowestResistance(cost_bonus.checked_div(4).unwrap_or(1).max(1))
                }
                PercentageIncreaseDamageAgainstLowestResistance(_) => {
                    PercentageIncreaseDamageAgainstLowestResistance(u16::try_from(cost_bonus.checked_div(4).unwrap_or(1).max(1)).unwrap_or(u16::MAX))
                }
                PercentageIncreaseTreasure(treasure_type, _) => {
                    PercentageIncreaseTreasure(treasure_type.clone(), u16::try_from(cost_bonus).unwrap_or(u16::MAX).max(1))
                }
                FlatIncreaseRewardedItems(_) => {
                    FlatIncreaseRewardedItems(u16::try_from(cost_bonus.checked_div(10).unwrap_or(1).max(1)).unwrap_or(u16::MAX))
                }
            }
        );
    }
    modifier_gain
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;

    use crate::game_generator::generate_testing_game;
    use crate::item_modifier::Modifier;
    use crate::item_resource::Type;
    use crate::modifier_cost::Cost;
    use crate::modifier_gain::Gain;
    use crate::roll_modifier::execute_craft;
    use crate::treasure_types::TreasureType;

    #[test]
    fn basic_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        execute_craft(&mut game.random_generator_state, &game.inventory[0].as_ref().unwrap().crafting_info);
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        let original_game = execute_craft(&mut game.random_generator_state, &game.inventory[0].as_ref().unwrap().crafting_info);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            let result = execute_craft(&mut game.random_generator_state, &game.inventory[0].as_ref().unwrap().crafting_info);
            assert_eq!(original_game, result);
        }
    }

    #[test]
    fn test_many_runs() {
        let mut game = generate_testing_game(Some([1; 16]));
        let mut cost_modifiers: HashMap<Cost, u32> = HashMap::new();
        let mut gain_modifiers: HashMap<Gain, u32> = HashMap::new();

        for _i in 1..1000 {
            let result = execute_craft(&mut game.random_generator_state, &game.inventory[0].as_ref().unwrap().crafting_info);

            setup_costs(&mut cost_modifiers, &result);

            setup_gains(&mut gain_modifiers, result);
        }

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| cost_modifiers.get(&Cost::FlatMinAttackRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| cost_modifiers.get(&Cost::FlatMaxAttackRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *cost_modifiers.get(&Cost::FlatSumMinAttackRequirement(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&Cost::FlatSumMaxAttackRequirement(0)).unwrap());

        assert_ne!(0, *cost_modifiers.get(&Cost::PlaceLimitedByIndexModulus(1, Vec::new())).unwrap());

        assert_eq!(0, Type::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&Cost::FlatItemResource(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, Type::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&Cost::FlatMinItemResourceRequirement(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, Type::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&Cost::FlatMaxItemResourceRequirement(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| cost_modifiers.get(&Cost::FlatMinResistanceRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| cost_modifiers.get(&Cost::FlatMaxResistanceRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *cost_modifiers.get(&Cost::FlatMinSumResistanceRequirement(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&Cost::FlatMaxSumResistanceRequirement(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&Cost::MinWinsInARow(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&Cost::MaxWinsInARow(0)).unwrap());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| gain_modifiers.get(&Gain::FlatDamage(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, Type::get_all().into_iter()
            .filter(|item_resource| gain_modifiers.get(&Gain::FlatItemResource(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| gain_modifiers.get(&Gain::PercentageIncreaseDamage(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| gain_modifiers.get(&Gain::FlatResistanceReduction(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .cloned()
            .filter(|attack_type| gain_modifiers.get(&Gain::PercentageIncreaseResistanceReduction(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *gain_modifiers.get(&Gain::FlatDamageAgainstHighestResistance(0)).unwrap());
        assert_ne!(0, *gain_modifiers.get(&Gain::PercentageIncreaseDamageAgainstHighestResistance(0)).unwrap());
        assert_ne!(0, *gain_modifiers.get(&Gain::FlatDamageAgainstLowestResistance(0)).unwrap());
        assert_ne!(0, *gain_modifiers.get(&Gain::PercentageIncreaseDamageAgainstLowestResistance(0)).unwrap());

        assert_eq!(0, TreasureType::get_all().into_iter()
            .filter(|treasure_type| gain_modifiers.get(&Gain::PercentageIncreaseTreasure(treasure_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *gain_modifiers.get(&Gain::FlatIncreaseRewardedItems(0)).unwrap());
    }

    fn setup_gains(gain_modifiers: &mut HashMap<Gain, u32>, result: Modifier) {
        for gain in result.gains {
            match gain {
                Gain::FlatItemResource(item_resource, _) => {
                    let token = Gain::FlatItemResource(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatDamage(attack_type, _) => {
                    let token = Gain::FlatDamage(attack_type, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseDamage(attack_type, _) => {
                    let token = Gain::PercentageIncreaseDamage(attack_type, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatResistanceReduction(item_resource, _) => {
                    let token = Gain::FlatResistanceReduction(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseResistanceReduction(item_resource, _) => {
                    let token = Gain::PercentageIncreaseResistanceReduction(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatDamageAgainstHighestResistance(_) => {
                    let token = Gain::FlatDamageAgainstHighestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseDamageAgainstHighestResistance(_) => {
                    let token = Gain::PercentageIncreaseDamageAgainstHighestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatDamageAgainstLowestResistance(_) => {
                    let token = Gain::FlatDamageAgainstLowestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseDamageAgainstLowestResistance(_) => {
                    let token = Gain::PercentageIncreaseDamageAgainstLowestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseTreasure(treasure_type, _) => {
                    let token = Gain::PercentageIncreaseTreasure(treasure_type, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatIncreaseRewardedItems(_) => {
                    let token = Gain::FlatIncreaseRewardedItems(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
            }
        }
    }

    fn setup_costs(cost_modifiers: &mut HashMap<Cost, u32>, result: &Modifier) {
        for cost in result.costs.clone() {
            match cost {
                Cost::FlatItemResource(item_resource, _) => {
                    let token = Cost::FlatItemResource(item_resource, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinItemResourceRequirement(item_resource, _) => {
                    let token = Cost::FlatMinItemResourceRequirement(item_resource, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxItemResourceRequirement(item_resource, _) => {
                    let token = Cost::FlatMaxItemResourceRequirement(item_resource, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinAttackRequirement(attack_type, _) => {
                    let token = Cost::FlatMinAttackRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxAttackRequirement(attack_type, _) => {
                    let token = Cost::FlatMaxAttackRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::PlaceLimitedByIndexModulus(_, _) => {
                    let token = Cost::PlaceLimitedByIndexModulus(1, Vec::new());
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatSumMinAttackRequirement(_) => {
                    let token = Cost::FlatSumMinAttackRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatSumMaxAttackRequirement(_) => {
                    let token = Cost::FlatSumMaxAttackRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinResistanceRequirement(attack_type, _) => {
                    let token = Cost::FlatMinResistanceRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxResistanceRequirement(attack_type, _) => {
                    let token = Cost::FlatMaxResistanceRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinSumResistanceRequirement(_) => {
                    let token = Cost::FlatMinSumResistanceRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxSumResistanceRequirement(_) => {
                    let token = Cost::FlatMaxSumResistanceRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::MinWinsInARow(_) => {
                    let token = Cost::MinWinsInARow(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::MaxWinsInARow(_) => {
                    let token = Cost::MaxWinsInARow(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
            }
        }
    }
}