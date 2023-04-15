use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::{Add, Div},
};

use rand::Rng;
use rand_pcg::Lcg64Xsh32;

use crate::the_world::{
    attack_types::{get_random_attack_type_from_unlocked, DamageType},
    item::CraftingInfo,
    item_modifier::Modifier,
    item_resource::Type,
    modifier_cost::Cost,
    modifier_gain::{
        Gain,
        Gain::{
            FlatDamage, FlatDamageAgainstHighestResistance, FlatDamageAgainstLowestResistance,
            FlatIncreaseRewardedItems, FlatItemResource, FlatResistanceReduction,
            PercentageIncreaseDamage, PercentageIncreaseDamageAgainstHighestResistance,
            PercentageIncreaseDamageAgainstLowestResistance, PercentageIncreaseResistanceReduction,
            PercentageIncreaseTreasure,
        },
    },
};

mod tests;

pub fn execute_craft(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
) -> Modifier {
    let minimum_elements = min(
        crafting_info.possible_rolls.min_resistance.len(),
        crafting_info.possible_rolls.min_simultaneous_resistances as usize,
    );
    let maximum_elements = min(
        crafting_info.possible_rolls.max_resistance.len(),
        crafting_info.possible_rolls.max_simultaneous_resistances as usize,
    );

    let (modifier_costs, cost) =
        execute_craft_roll_modifier_costs(random_generator_state, crafting_info);

    let modifier_gain = execute_craft_roll_modifier_benefits(
        random_generator_state,
        crafting_info,
        cost,
        minimum_elements,
        maximum_elements,
    );

    Modifier {
        costs: modifier_costs,
        gains: modifier_gain,
    }
}

fn execute_craft_roll_modifier_costs(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
) -> (Vec<Cost>, u64) {
    let mut modifier_costs = Vec::new();
    let mut accumulated_cost = 0;
    let max_cost = crafting_info
        .possible_rolls
        .max_resistance
        .values()
        .sum::<u64>()
        / u64::from(crafting_info.possible_rolls.max_simultaneous_resistances);

    //TODO unblocked damage will apply unique effect

    let number_of_costs = random_generator_state.gen_range(
        0..crafting_info
            .possible_rolls
            .max_simultaneous_resistances
            .add(1),
    );

    for _i in 0..number_of_costs {
        if accumulated_cost < max_cost {
            accumulated_cost += match random_generator_state.gen_range(0..14) {
                0 => {
                    add_flat_min_attack(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                1 => {
                    add_flat_max_attack(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                2 => {
                    add_place_limited_by_index_modulus(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                    )
                }
                3 => {
                    add_flat_sum_min_attach(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                4 => {
                    add_flat_sum_max_attack(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                5 => {
                    add_flat_min_item_resource(
                        random_generator_state,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                6 => {
                    add_flat_max_item_resource(
                        random_generator_state,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                7 => {
                    add_min_resistance(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                8 => {
                    add_flat_max_resistance(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                9 => {
                    add_flat_min_sum_resistance(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                10 => {
                    add_flat_max_sum_resistance(
                        random_generator_state,
                        crafting_info,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
                11 => {
                    add_min_wins_in_row(random_generator_state, crafting_info, &mut modifier_costs)
                }
                12 => {
                    add_max_wins_in_row(random_generator_state, crafting_info, &mut modifier_costs)
                }
                _ => {
                    add_flat_item_resource(
                        random_generator_state,
                        &mut modifier_costs,
                        accumulated_cost,
                        max_cost,
                    )
                }
            }
        }
    }

    (modifier_costs, accumulated_cost)
}

fn add_flat_min_attack(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(
        random_generator_state,
        &crafting_info.possible_rolls.min_resistance,
    );

    let minimum_value = *crafting_info
        .possible_rolls
        .min_resistance
        .get(&attack_type)
        .unwrap();
    let maximum_value = *crafting_info
        .possible_rolls
        .max_resistance
        .get(&attack_type)
        .unwrap();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatMinAttackRequirement(attack_type, value));
    value
}

fn add_flat_max_attack(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(
        random_generator_state,
        &crafting_info.possible_rolls.min_resistance,
    );

    let minimum_value = *crafting_info
        .possible_rolls
        .min_resistance
        .get(&attack_type)
        .unwrap();
    let maximum_value = *crafting_info
        .possible_rolls
        .max_resistance
        .get(&attack_type)
        .unwrap();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatMaxAttackRequirement(attack_type, value));
    maximum_value - value
}

fn add_place_limited_by_index_modulus(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
) -> u64 {
    let max_modulus = min(usize::from(u8::MAX), crafting_info.places_count);
    let max_modulus = max(3, max_modulus);
    let modulus = u8::try_from(random_generator_state.gen_range(2..max_modulus)).unwrap();
    let number_of_valid_values = random_generator_state.gen_range(1..modulus);

    let valid_numbers = (0..number_of_valid_values)
        .map(|_| random_generator_state.gen_range(0..modulus))
        .collect::<HashSet<u8>>()
        .into_iter()
        .collect();

    modifier_costs.push(Cost::PlaceLimitedByIndexModulus(modulus, valid_numbers));

    (crafting_info.places_count * ((modulus / number_of_valid_values) as usize)) as u64
}

fn add_flat_sum_min_attach(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let minimum_value = crafting_info
        .possible_rolls
        .min_resistance
        .values()
        .sum::<u64>();
    let maximum_value = crafting_info
        .possible_rolls
        .max_resistance
        .values()
        .sum::<u64>();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatSumMinAttackRequirement(value));
    value
}

fn add_flat_sum_max_attack(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let minimum_value = crafting_info
        .possible_rolls
        .min_resistance
        .values()
        .sum::<u64>();
    let maximum_value = crafting_info
        .possible_rolls
        .max_resistance
        .values()
        .sum::<u64>();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatSumMaxAttackRequirement(value));
    maximum_value - value
}

fn add_flat_min_item_resource(
    random_generator_state: &mut Lcg64Xsh32,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let cost = random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
    modifier_costs.push(Cost::FlatMinItemResourceRequirement(Type::Mana, cost));
    cost
}

fn add_flat_max_item_resource(
    random_generator_state: &mut Lcg64Xsh32,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let cost = random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
    modifier_costs.push(Cost::FlatMaxItemResourceRequirement(Type::Mana, cost));
    (max_cost - accumulated_cost) - cost //TODO Better cost
}

fn add_min_resistance(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(
        random_generator_state,
        &crafting_info.possible_rolls.min_resistance,
    );

    let minimum_value = *crafting_info
        .possible_rolls
        .min_resistance
        .get(&attack_type)
        .unwrap();
    let maximum_value = *crafting_info
        .possible_rolls
        .max_resistance
        .get(&attack_type)
        .unwrap();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatMinResistanceRequirement(attack_type, value));
    value
}

fn add_flat_max_resistance(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let attack_type = get_random_attack_type_from_unlocked(
        random_generator_state,
        &crafting_info.possible_rolls.min_resistance,
    );

    let minimum_value = *crafting_info
        .possible_rolls
        .min_resistance
        .get(&attack_type)
        .unwrap();
    let maximum_value = *crafting_info
        .possible_rolls
        .max_resistance
        .get(&attack_type)
        .unwrap();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatMaxResistanceRequirement(attack_type, value));
    maximum_value - value
}

fn add_flat_min_sum_resistance(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let minimum_value = crafting_info
        .possible_rolls
        .min_resistance
        .values()
        .sum::<u64>();
    let maximum_value = crafting_info
        .possible_rolls
        .max_resistance
        .values()
        .sum::<u64>();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatMinSumResistanceRequirement(value));
    value
}

fn add_flat_max_sum_resistance(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let minimum_value = crafting_info
        .possible_rolls
        .min_resistance
        .values()
        .sum::<u64>();
    let maximum_value = crafting_info
        .possible_rolls
        .max_resistance
        .values()
        .sum::<u64>();
    let value = min(
        max_cost - accumulated_cost,
        random_generator_state.gen_range(minimum_value..=maximum_value),
    );

    modifier_costs.push(Cost::FlatMaxSumResistanceRequirement(value));
    maximum_value - value
}

fn add_min_wins_in_row(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
) -> u64 {
    let value = u8::try_from(min(usize::from(u8::MAX), crafting_info.places_count * 2)).unwrap();
    let value = max(2, value);
    let value = random_generator_state.gen_range(1..value);

    modifier_costs.push(Cost::MinWinsInARow(value));
    u64::from(value)
}

fn add_max_wins_in_row(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    modifier_costs: &mut Vec<Cost>,
) -> u64 {
    let max_value =
        u8::try_from(min(usize::from(u8::MAX), crafting_info.places_count * 2)).unwrap();
    let value = max(1, max_value);
    let value = random_generator_state.gen_range(0..value);

    modifier_costs.push(Cost::MaxWinsInARow(value));
    u64::from(max_value - value)
}

fn add_flat_item_resource(
    random_generator_state: &mut Lcg64Xsh32,
    modifier_costs: &mut Vec<Cost>,
    accumulated_cost: u64,
    max_cost: u64,
) -> u64 {
    let cost = random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
    modifier_costs.push(Cost::FlatItemResource(Type::Mana, cost));
    cost
}

fn execute_craft_roll_modifier_benefits(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    cost: u64,
    minimum_elements: usize,
    maximum_elements: usize,
) -> Vec<Gain> {
    let attack_types = DamageType::get_all()
        .into_iter()
        .filter(|attack_type| {
            crafting_info
                .possible_rolls
                .min_resistance
                .contains_key(attack_type)
        })
        .collect();

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
        modifier_gain.push(match &all_modifier_gain_options[modifier_index] {
            //TODO do the same with costs.
            FlatDamage(attack_type, _) => {
                let damage = randomize_flat_damage(
                    random_generator_state,
                    crafting_info,
                    cost_bonus,
                    attack_type,
                );
                FlatDamage(attack_type.clone(), damage)
            }
            PercentageIncreaseDamage(attack_type, _) => {
                PercentageIncreaseDamage(
                    attack_type.clone(),
                    u16::try_from(cost_bonus).unwrap_or(u16::MAX).max(1),
                )
            }
            FlatItemResource(item_resource_type, _) => {
                FlatItemResource(item_resource_type.clone(), max(1, cost_bonus * 2))
            }
            FlatResistanceReduction(attack_type, _) => {
                let damage = randomize_flat_damage(
                    random_generator_state,
                    crafting_info,
                    cost_bonus,
                    attack_type,
                );
                FlatResistanceReduction(attack_type.clone(), damage)
            }
            PercentageIncreaseResistanceReduction(attack_type, _) => {
                PercentageIncreaseResistanceReduction(
                    attack_type.clone(),
                    u16::try_from(cost_bonus).unwrap_or(u16::MAX).max(1),
                )
            }
            FlatDamageAgainstHighestResistance(_) => {
                FlatDamageAgainstHighestResistance(cost_bonus.checked_div(2).unwrap_or(1).max(1))
            }
            PercentageIncreaseDamageAgainstHighestResistance(_) => {
                PercentageIncreaseDamageAgainstHighestResistance(
                    u16::try_from(cost_bonus.checked_div(2).unwrap_or(1).max(1))
                        .unwrap_or(u16::MAX),
                )
            }
            FlatDamageAgainstLowestResistance(_) => {
                FlatDamageAgainstLowestResistance(cost_bonus.checked_div(4).unwrap_or(1).max(1))
            }
            PercentageIncreaseDamageAgainstLowestResistance(_) => {
                PercentageIncreaseDamageAgainstLowestResistance(
                    u16::try_from(cost_bonus.checked_div(4).unwrap_or(1).max(1))
                        .unwrap_or(u16::MAX),
                )
            }
            PercentageIncreaseTreasure(treasure_type, _) => {
                PercentageIncreaseTreasure(
                    treasure_type.clone(),
                    u16::try_from(cost_bonus).unwrap_or(u16::MAX).max(1),
                )
            }
            FlatIncreaseRewardedItems(_) => {
                FlatIncreaseRewardedItems(
                    u16::try_from(cost_bonus.checked_div(10).unwrap_or(1).max(1))
                        .unwrap_or(u16::MAX),
                )
            }
        });
    }
    modifier_gain
}

fn randomize_flat_damage(
    random_generator_state: &mut Lcg64Xsh32,
    crafting_info: &CraftingInfo,
    cost_bonus: u64,
    attack_type: &DamageType,
) -> u64 {
    let min_damage = *crafting_info
        .possible_rolls
        .min_resistance
        .get(attack_type)
        .unwrap_or(&0);
    let max_damage = *crafting_info
        .possible_rolls
        .max_resistance
        .get(attack_type)
        .unwrap_or(&1);
    random_generator_state
        .gen_range(min_damage..=max_damage)
        .div(2)
        .max(1)
        .checked_mul(cost_bonus * 2)
        .unwrap_or(u64::MAX)
}
