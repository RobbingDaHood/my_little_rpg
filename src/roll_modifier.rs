use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::Add;
use rand::Rng;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::CraftingInfo;
use crate::item_modifier::ItemModifier;
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;
use crate::modifier_gain::ModifierGain::{FlatDamageAgainstHighestResistance, FlatItemResource, FlatResistanceReduction, PercentageIncreaseDamage, PercentageIncreaseResistanceReduction};
use crate::modifier_gain::ModifierGain;
use crate::modifier_gain::ModifierGain::FlatDamage;
use crate::game::get_random_attack_type_from_unlocked;

pub fn execute_craft_roll_modifier(game: &mut Game, item_index: usize) -> ItemModifier {
    let crafting_info = &game.inventory[item_index].as_ref().unwrap().crafting_info.clone();

    let minimum_elements = min(crafting_info.possible_rolls.min_resistance.len(), crafting_info.possible_rolls.min_simultaneous_resistances as usize);
    let maximum_elements = min(crafting_info.possible_rolls.max_resistance.len(), crafting_info.possible_rolls.max_simultaneous_resistances as usize);

    let (modifier_costs, cost) = execute_craft_roll_modifier_costs(game, crafting_info);

    let modifier_gain = execute_craft_roll_modifier_benefits(game, crafting_info, cost, minimum_elements, maximum_elements);

    let new_item_modifier = ItemModifier {
        costs: modifier_costs,
        gains: modifier_gain,
    };
    new_item_modifier
}

fn execute_craft_roll_modifier_costs(game: &mut Game, crafting_info: &CraftingInfo) -> (Vec<ModifierCost>, u64) {
    let mut modifier_costs = Vec::new();
    let mut accumulated_cost = 0;
    let max_cost = crafting_info.possible_rolls.max_resistance.values().sum::<u64>() / crafting_info.possible_rolls.max_simultaneous_resistances as u64;

    //TODO unblocked damage will applay unique effect

    let number_of_costs = game.random_generator_state.gen_range(0..crafting_info.possible_rolls.max_simultaneous_resistances.add(1));

    for _i in 0..number_of_costs {
        if accumulated_cost < max_cost {
            match game.random_generator_state.gen_range(0..14) {
                0 => {
                    let attack_type = get_random_attack_type_from_unlocked(game, &Some(&crafting_info.possible_rolls));

                    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
                    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatMinAttackRequirement(attack_type, value.clone()));
                    accumulated_cost += value;
                }
                1 => {
                    let attack_type = get_random_attack_type_from_unlocked(game, &Some(&crafting_info.possible_rolls));

                    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
                    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatMaxAttackRequirement(attack_type, value.clone()));
                    accumulated_cost += maximum_value - value;
                }
                2 => {
                    let max_modulus = min(usize::from(u8::MAX), game.places.len()); //TODO Make dependent on item possible rolls instead, store the number of places there
                    let max_modulus = max(3, max_modulus);
                    let modulus = u8::try_from(game.random_generator_state.gen_range(2..max_modulus)).unwrap();
                    let number_of_valid_values = game.random_generator_state.gen_range(1..modulus);

                    let valid_numbers = (0..number_of_valid_values).into_iter()
                        .map(|_| game.random_generator_state.gen_range(0..modulus))
                        .map(|value| u8::try_from(value).unwrap())
                        .collect::<HashSet<u8>>().into_iter()
                        .collect();

                    modifier_costs.push(ModifierCost::PlaceLimitedByIndexModulus(modulus, valid_numbers));

                    accumulated_cost += (game.places.len() * ((modulus / number_of_valid_values) as usize)) as u64;
                }
                3 => {
                    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
                    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatSumMinAttackRequirement(value.clone()));
                    accumulated_cost += value;
                }
                4 => {
                    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
                    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatSumMaxAttackRequirement(value.clone()));
                    accumulated_cost += maximum_value - value;
                }
                5 => {
                    let cost = game.random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
                    modifier_costs.push(ModifierCost::FlatMinItemResourceRequirement(ItemResourceType::Mana, cost));
                    accumulated_cost += cost; //TODO Better cost
                }
                6 => {
                    let cost = game.random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
                    modifier_costs.push(ModifierCost::FlatMaxItemResourceRequirement(ItemResourceType::Mana, cost));
                    accumulated_cost += (max_cost - accumulated_cost) - cost; //TODO Better cost
                }
                7 => {
                    let attack_type = get_random_attack_type_from_unlocked(game, &Some(&crafting_info.possible_rolls));

                    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
                    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatMinResistanceRequirement(attack_type, value.clone()));
                    accumulated_cost += value;
                }
                8 => {
                    let attack_type = get_random_attack_type_from_unlocked(game, &Some(&crafting_info.possible_rolls));

                    let minimum_value = *crafting_info.possible_rolls.min_resistance.get(&attack_type).unwrap();
                    let maximum_value = *crafting_info.possible_rolls.max_resistance.get(&attack_type).unwrap();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatMaxResistanceRequirement(attack_type, value.clone()));
                    accumulated_cost += maximum_value - value;
                }
                9 => {
                    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
                    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatMinSumResistanceRequirement(value.clone()));
                    accumulated_cost += value;
                }
                10 => {
                    let minimum_value = crafting_info.possible_rolls.min_resistance.values().sum::<u64>();
                    let maximum_value = crafting_info.possible_rolls.max_resistance.values().sum::<u64>();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..=maximum_value));

                    modifier_costs.push(ModifierCost::FlatMaxSumResistanceRequirement(value.clone()));
                    accumulated_cost += maximum_value - value;
                }
                11 => {
                    let value = u8::try_from(min(usize::from(u8::MAX), game.places.len() * 2)).unwrap();
                    let value = max(2, value);
                    let value = game.random_generator_state.gen_range(1..value);

                    modifier_costs.push(ModifierCost::MinWinsInARow(value.clone()));
                    accumulated_cost += u64::from(value);
                }
                12 => {
                    let max_value = u8::try_from(min(usize::from(u8::MAX), game.places.len() * 2)).unwrap();
                    let value = max(1, max_value);
                    let value = game.random_generator_state.gen_range(0..value);

                    modifier_costs.push(ModifierCost::MaxWinsInARow(value.clone()));
                    accumulated_cost += u64::from(max_value - value);
                }
                _ => {
                    let cost = game.random_generator_state.gen_range(1..max(2, max_cost - accumulated_cost));
                    modifier_costs.push(ModifierCost::FlatItemResource(ItemResourceType::Mana, cost));
                    accumulated_cost += cost;
                }
            }
        }
    }

    (modifier_costs, accumulated_cost)
}

fn execute_craft_roll_modifier_benefits(game: &mut Game, crafting_info: &CraftingInfo, cost: u64, minimum_elements: usize, maximum_elements: usize) -> Vec<ModifierGain> {
    let attack_types = AttackType::get_all().iter()
        .filter(|attack_type| crafting_info.possible_rolls.min_resistance.contains_key(attack_type))
        .map(|attack_type| attack_type.clone())
        .collect::<Vec<AttackType>>();

    let mut leftover_cost = cost;

    let all_modifier_gain_options = ModifierGain::get_all_given_attack_types(attack_types);
    let mut modifier_gain = Vec::new();
    for i in minimum_elements..=maximum_elements {
        let cost_bonus = if i == maximum_elements {
            leftover_cost
        } else {
            game.random_generator_state.gen_range(0..=leftover_cost)
        };
        leftover_cost -= cost_bonus;

        let gain_seize = all_modifier_gain_options.len();
        let modifier_index = game.random_generator_state.gen_range(0..gain_seize);
        modifier_gain.push(
            match &all_modifier_gain_options[modifier_index] { //TODO do the same with costs.
                FlatDamage(attack_type, _) => {
                    let min_damage = *crafting_info.possible_rolls.min_resistance.get(attack_type).unwrap_or(&0);
                    let max_damage = *crafting_info.possible_rolls.max_resistance.get(attack_type).unwrap_or(&1);
                    let damage = game.random_generator_state.gen_range(min_damage..=max_damage);
                    let damage = damage / 2;
                    let damage = max(1, damage);
                    let damage = damage + cost_bonus * 2;

                    ModifierGain::FlatDamage(attack_type.clone(), damage.clone())
                }
                PercentageIncreaseDamage(attack_type, _) => {
                    ModifierGain::PercentageIncreaseDamage(attack_type.clone(), u16::try_from(cost_bonus).unwrap_or(u16::MAX))
                }
                FlatItemResource(item_resource_type, _) => {
                    ModifierGain::FlatItemResource(item_resource_type.clone(), cost_bonus * 2 + 1)
                }
                FlatResistanceReduction(attack_type, _) => {
                    let min_damage = *crafting_info.possible_rolls.min_resistance.get(attack_type).unwrap_or(&0);
                    let max_damage = *crafting_info.possible_rolls.max_resistance.get(attack_type).unwrap_or(&1);
                    let damage = game.random_generator_state.gen_range(min_damage..=max_damage);
                    let damage = damage / 2;
                    let damage = max(1, damage);
                    let damage = damage + cost_bonus * 2;

                    ModifierGain::FlatResistanceReduction(attack_type.clone(), damage.clone())
                }
                PercentageIncreaseResistanceReduction(attack_type, _) => {
                    ModifierGain::PercentageIncreaseResistanceReduction(attack_type.clone(), u16::try_from(cost_bonus).unwrap_or(u16::MAX))
                }
                FlatDamageAgainstHighestResistance(_) => {
                    ModifierGain::FlatDamageAgainstHighestResistance(cost_bonus.checked_div(2).unwrap_or(1))
                }
            }
        )
    }
    modifier_gain
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;
    use crate::game_generator::{generate_new_game, generate_testing_game};
    use crate::item_resource::ItemResourceType;
    use crate::modifier_cost::ModifierCost;
    use crate::modifier_gain::ModifierGain;
    use crate::roll_modifier::execute_craft_roll_modifier;


    #[test]
    fn basic_test() {
        let mut game = generate_new_game(Some([1; 16]));
        game.inventory.push(Some(game.equipped_items[0].clone()));
        execute_craft_roll_modifier(&mut game, 0);
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        let original_game = execute_craft_roll_modifier(&mut game, 0);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            let result = execute_craft_roll_modifier(&mut game, 0);
            assert_eq!(original_game, result);
        }
    }

    #[test]
    fn test_many_runs() {
        let mut game = generate_testing_game(Some([1; 16]));
        let mut cost_modifiers: HashMap<ModifierCost, u32> = HashMap::new();
        let mut gain_modifiers: HashMap<ModifierGain, u32> = HashMap::new();

        for _i in 1..1000 {
            let result = execute_craft_roll_modifier(&mut game, 0);

            for cost in result.costs {
                match cost {
                    ModifierCost::FlatItemResource(item_resource, _) => {
                        let token = ModifierCost::FlatItemResource(item_resource, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMinItemResourceRequirement(item_resource, _) => {
                        let token = ModifierCost::FlatMinItemResourceRequirement(item_resource, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMaxItemResourceRequirement(item_resource, _) => {
                        let token = ModifierCost::FlatMaxItemResourceRequirement(item_resource, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMinAttackRequirement(attack_type, _) => {
                        let token = ModifierCost::FlatMinAttackRequirement(attack_type, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMaxAttackRequirement(attack_type, _) => {
                        let token = ModifierCost::FlatMaxAttackRequirement(attack_type, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::PlaceLimitedByIndexModulus(_, _) => {
                        let token = ModifierCost::PlaceLimitedByIndexModulus(1, Vec::new());
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatSumMinAttackRequirement(_) => {
                        let token = ModifierCost::FlatSumMinAttackRequirement(0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatSumMaxAttackRequirement(_) => {
                        let token = ModifierCost::FlatSumMaxAttackRequirement(0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMinResistanceRequirement(attack_type, _) => {
                        let token = ModifierCost::FlatMinResistanceRequirement(attack_type, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMaxResistanceRequirement(attack_type, _) => {
                        let token = ModifierCost::FlatMaxResistanceRequirement(attack_type, 0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMinSumResistanceRequirement(_) => {
                        let token = ModifierCost::FlatMinSumResistanceRequirement(0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::FlatMaxSumResistanceRequirement(_) => {
                        let token = ModifierCost::FlatMaxSumResistanceRequirement(0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::MinWinsInARow(_) => {
                        let token = ModifierCost::MinWinsInARow(0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierCost::MaxWinsInARow(_) => {
                        let token = ModifierCost::MaxWinsInARow(0);
                        *cost_modifiers.entry(token).or_insert(0) += 1;
                    }
                }
            }

            for gain in result.gains {
                match gain {
                    ModifierGain::FlatItemResource(item_resource, _) => {
                        let token = ModifierGain::FlatItemResource(item_resource, 0);
                        *gain_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierGain::FlatDamage(attack_type, _) => {
                        let token = ModifierGain::FlatDamage(attack_type, 0);
                        *gain_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierGain::PercentageIncreaseDamage(attack_type, _) => {
                        let token = ModifierGain::PercentageIncreaseDamage(attack_type, 0);
                        *gain_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierGain::FlatResistanceReduction(item_resource, _) => {
                        let token = ModifierGain::FlatResistanceReduction(item_resource, 0);
                        *gain_modifiers.entry(token).or_insert(0) += 1;
                    }
                        ModifierGain::PercentageIncreaseResistanceReduction(item_resource, _) => {
                    let token = ModifierGain::PercentageIncreaseResistanceReduction(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                    }
                    ModifierGain::FlatDamageAgainstHighestResistance(_) => {
                        let token = ModifierGain::FlatDamageAgainstHighestResistance(0);
                        *gain_modifiers.entry(token).or_insert(0) += 1;
                    }
                }
            }
        }

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| cost_modifiers.get(&ModifierCost::FlatMinAttackRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| cost_modifiers.get(&ModifierCost::FlatMaxAttackRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *cost_modifiers.get(&ModifierCost::FlatSumMinAttackRequirement(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&ModifierCost::FlatSumMaxAttackRequirement(0)).unwrap());

        assert_ne!(0, *cost_modifiers.get(&ModifierCost::PlaceLimitedByIndexModulus(1, Vec::new())).unwrap());

        assert_eq!(0, ItemResourceType::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&ModifierCost::FlatItemResource(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, ItemResourceType::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&ModifierCost::FlatMinItemResourceRequirement(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, ItemResourceType::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&ModifierCost::FlatMaxItemResourceRequirement(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| cost_modifiers.get(&ModifierCost::FlatMinResistanceRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| cost_modifiers.get(&ModifierCost::FlatMaxResistanceRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *cost_modifiers.get(&ModifierCost::FlatMinSumResistanceRequirement(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&ModifierCost::FlatMaxSumResistanceRequirement(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&ModifierCost::MinWinsInARow(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&ModifierCost::MaxWinsInARow(0)).unwrap());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| gain_modifiers.get(&ModifierGain::FlatDamage(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, ItemResourceType::get_all().into_iter()
            .filter(|item_resource| gain_modifiers.get(&ModifierGain::FlatItemResource(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| gain_modifiers.get(&ModifierGain::PercentageIncreaseDamage(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| gain_modifiers.get(&ModifierGain::FlatResistanceReduction(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| gain_modifiers.get(&ModifierGain::PercentageIncreaseResistanceReduction(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_ne!(0, *gain_modifiers.get(&ModifierGain::FlatDamageAgainstHighestResistance(0)).unwrap());

    }
}