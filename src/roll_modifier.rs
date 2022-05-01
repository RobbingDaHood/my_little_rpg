use std::cmp::{max, min};
use std::ops::Div;
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

pub fn execute_craft_roll_modifier(game: &mut Game, item_index: usize) -> ItemModifier {
    let crafting_info = &game.inventory[item_index].crafting_info.clone();

    let minimum_elements = min(crafting_info.possible_rolls.min_resistance.len(), crafting_info.possible_rolls.min_simultaneous_resistances as usize);
    let maximum_elements = min(crafting_info.possible_rolls.max_resistance.len(), crafting_info.possible_rolls.max_simultaneous_resistances as usize);

    let (modifier_costs, cost) = execute_craft_roll_modifier_costs(game, crafting_info, minimum_elements, maximum_elements);

    let modifier_gain = execute_craft_roll_modifier_benefits(game, crafting_info, cost, minimum_elements, maximum_elements);

    let new_item_modifier = ItemModifier {
        costs: modifier_costs,
        gains: modifier_gain,
    };
    new_item_modifier
}

fn execute_craft_roll_modifier_costs(game: &mut Game, crafting_info: &CraftingInfo, minimum_elements: usize, maximum_elements: usize) -> (Vec<ModifierCost>, u64) {
    let mut modifier_costs = Vec::new();
    let mut cost = 0;

    //TODO Add new costs. Also in a more safe way.

    let maximum_amount_of_costs = max(2, minimum_elements.div(2));

    for i in 1..=maximum_amount_of_costs {
        if game.random_generator_state.gen_range(0..i) != 0 {
            let average_max = crafting_info.possible_rolls.max_resistance.values().sum::<u64>() / maximum_elements as u64;
            cost += game.random_generator_state.gen_range(1..average_max);
        }
    }

    if cost != 0 {
        modifier_costs.push(ModifierCost::FlatItemResource(ItemResourceType::Mana, cost));
    }

    (modifier_costs, cost)
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
        let modifier_index = game.random_generator_state.gen_range(0..gain_seize); //TODO this does not respect the seed. Could be because there is more rolls earlier
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
    use crate::game_generator::generate_testing_game;
    use crate::roll_modifier::execute_craft_roll_modifier;

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

        for _i in 1..1000 {
            execute_craft_roll_modifier(&mut game, 0);
        }
    }
}