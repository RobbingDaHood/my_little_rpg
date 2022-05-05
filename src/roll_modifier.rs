use std::cmp::{max, min};
use std::ops::Add;
use rand::Rng;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::CraftingInfo;
use crate::item_modifier::ItemModifier;
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;
use crate::modifier_gain::ModifierGain::FlatItemResource;
use crate::modifier_gain::ModifierGain;
use crate::modifier_gain::ModifierGain::FlatDamage;
use rand::prelude::SliceRandom;

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
            match game.random_generator_state.gen_range(0..2) {
                0 => {
                    let attack_type = AttackType::get_all().into_iter()
                        .filter(|attack_type| game.difficulty.min_resistance.contains_key(attack_type))
                        .map(|attack_type| attack_type.clone())
                        .collect::<Vec<AttackType>>()
                        .choose(&mut game.random_generator_state)
                        .unwrap()
                        .clone();

                    let minimum_value = *game.difficulty.min_resistance.get(&attack_type).unwrap();
                    let maximum_value = *game.difficulty.max_resistance.get(&attack_type).unwrap();
                    let value = min(max_cost - accumulated_cost, game.random_generator_state.gen_range(minimum_value..maximum_value));

                    modifier_costs.push(ModifierCost::FlatMinAttackRequirement(attack_type, value.clone()));
                    accumulated_cost += value;
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
            match &all_modifier_gain_options[modifier_index] {
                FlatDamage(attack_type, _) => {
                    let min_damage = *crafting_info.possible_rolls.min_resistance.get(attack_type).unwrap_or(&0);
                    let max_damage = *crafting_info.possible_rolls.max_resistance.get(attack_type).unwrap_or(&1);
                    let damage = game.random_generator_state.gen_range(min_damage..=max_damage);
                    let damage = damage / 2;
                    let damage = max(1, damage);
                    let damage = damage + cost_bonus * 2;


                    ModifierGain::FlatDamage(attack_type.clone(), damage.clone())
                }
                FlatItemResource(item_resource_type, _) => {
                    ModifierGain::FlatItemResource(item_resource_type.clone(), cost_bonus * 2 + 1)
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
                    ModifierCost::FlatMinAttackRequirement(attack_type, _) => {
                        let token = ModifierCost::FlatMinAttackRequirement(attack_type, 0);
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
                }
            }
        }

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| cost_modifiers.get(&ModifierCost::FlatMinAttackRequirement(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, ItemResourceType::get_all().into_iter()
            .filter(|item_resource| cost_modifiers.get(&ModifierCost::FlatItemResource(item_resource.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, game.difficulty.min_resistance.keys()
            .map(|attack_type| attack_type.clone())
            .filter(|attack_type| gain_modifiers.get(&ModifierGain::FlatDamage(attack_type.clone(), 0)).unwrap() == &0)
            .count());

        assert_eq!(0, ItemResourceType::get_all().into_iter()
            .filter(|item_resource| gain_modifiers.get(&ModifierGain::FlatItemResource(item_resource.clone(), 0)).unwrap() == &0)
            .count());
    }
}