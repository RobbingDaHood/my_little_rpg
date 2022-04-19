use std::cmp::{max, min};
use std::ops::Div;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item_modifier::ItemModifier;
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;
use crate::modifier_gain::ModifierGain::FlatItemResource;
use crate::modifier_gain::ModifierGain;
use crate::modifier_gain::ModifierGain::FlatDamage;

pub fn execute_craft_roll_modifier(game: &mut Game) -> ItemModifier {
    let minimum_elements = min(game.place_generator_input.min_resistance.len(), game.place_generator_input.min_simultaneous_resistances as usize);
    let maximum_elements = min(game.place_generator_input.max_resistance.len(), game.place_generator_input.max_simultaneous_resistances as usize);

    let mut rng = rand::thread_rng();

    let (modifier_costs, cost) = execute_craft_roll_modifier_costs(game, &mut rng, minimum_elements, maximum_elements);

    let modifier_gain = execute_craft_roll_modifier_benefits(game, &mut rng, cost, minimum_elements, maximum_elements);

    let new_item_modifier = ItemModifier {
        costs: modifier_costs,
        gains: modifier_gain,
    };
    new_item_modifier
}

fn execute_craft_roll_modifier_costs(game: &Game, rng: &mut ThreadRng, minimum_elements: usize, maximum_elements: usize) -> (Vec<ModifierCost>, u64) {
    let mut modifier_costs = Vec::new();
    let mut cost = 0;

    let maximum_amount_of_costs = max(2, minimum_elements.div(2));

    for i in 1..=maximum_amount_of_costs {
        if rng.gen_range(0..i) != 0 {
            let average_max = game.place_generator_input.max_resistance.values().sum::<u64>() / maximum_elements as u64;
            cost += rng.gen_range(1..average_max);
        }
    }

    if cost != 0 {
        modifier_costs.push(ModifierCost::FlatItemResource(ItemResourceType::Mana, cost));
    }

    (modifier_costs, cost)
}

fn execute_craft_roll_modifier_benefits(game: &mut Game, rng: &mut ThreadRng, cost: u64, minimum_elements: usize, maximum_elements: usize) -> Vec<ModifierGain> {
    let attack_types = game.place_generator_input.min_resistance.iter()
        .map(|(attack_type, _)| attack_type.clone())
        .collect::<Vec<AttackType>>();

    let mut leftover_cost = cost;

    let all_modifier_gain_options = ModifierGain::get_all_given_attack_types(attack_types);
    let mut modifier_gain = Vec::new();
    for i in minimum_elements..=maximum_elements {
        let cost_bonus = if i == maximum_elements {
            leftover_cost
        } else {
            rng.gen_range(0..=leftover_cost)
        };
        leftover_cost -= cost_bonus;

        modifier_gain.push(
            match &all_modifier_gain_options[rng.gen_range(0..all_modifier_gain_options.len())] {
                FlatDamage(attack_type, _) => {
                    let min_damage = *game.place_generator_input.min_resistance.get(attack_type).unwrap_or(&0);
                    let max_damage = *game.place_generator_input.max_resistance.get(attack_type).unwrap_or(&1);
                    let damage = rng.gen_range(min_damage..max_damage + 1);
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

//TODO tests