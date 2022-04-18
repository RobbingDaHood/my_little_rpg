use std::cmp;
use std::cmp::{max, min};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::Game;
use crate::item::Item;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::item_modifier::ItemModifier;
use crate::item_resource::ItemResourceType;
use crate::modifier_cost::ModifierCost;
use crate::modifier_gain::ModifierGain;
use crate::modifier_gain::ModifierGain::{FlatDamage, FlatItemResource};

pub fn execute_craft_reroll_modifier(game: &mut Game, inventory_index: usize, modifier_index: usize) -> Result<Item, String> {
    //validation
    if game.inventory.len() <= inventory_index {
        return Err(format!("inventory_index {} is not within the range of the inventory {}", inventory_index, game.inventory.len()));
    }
    if game.inventory[inventory_index].modifiers.len() <= modifier_index {
        return Err(format!("modifier_index {} is not within the range of the item modifiers {}", inventory_index, game.inventory[inventory_index].modifiers.len()));
    }

    //Crafting cost
    let crafting_cost = (game.inventory[inventory_index].modifiers.len() * (modifier_index + 1) * 5) as u64;
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for reroll_modifier, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Create item
    let mut rng = rand::thread_rng();

    let (modifier_costs, cost) = execute_craft_reroll_modifier_costs(&mut rng);

    let modifier_gain = execute_craft_reroll_modifier_benefits(game, &mut rng, cost);

    let new_item_modifier = ItemModifier {
        costs: modifier_costs,
        gains: modifier_gain,
    };

    game.inventory[inventory_index].modifiers[modifier_index] = new_item_modifier;

    Ok(game.inventory[inventory_index].clone())
}

fn execute_craft_reroll_modifier_benefits(game: &mut Game, rng: &mut ThreadRng, cost: u64) -> Vec<ModifierGain> {
    let attack_types = game.place_generator_input.min_resistance.iter()
        .map(|(attack_type, _)| attack_type.clone())
        .collect::<Vec<AttackType>>();

    let all_modifier_gain_options = ModifierGain::get_all_given_attack_types(attack_types);
    let modifier_gain = vec![
        match &all_modifier_gain_options[rng.gen_range(0..all_modifier_gain_options.len())] {
            FlatDamage(attack_type, _) => {
                let min_damage = *game.place_generator_input.min_resistance.get(attack_type).unwrap_or(&0);
                let max_damage = *game.place_generator_input.max_resistance.get(attack_type).unwrap_or(&1);
                let damage = rng.gen_range(min_damage..max_damage + 1);
                let damage = damage / 2;
                let damage = damage + cost;
                let damage = max(1, damage);

                ModifierGain::FlatDamage(attack_type.clone(), damage.clone())
            }
            FlatItemResource(item_resource_type, _) => {
                ModifierGain::FlatItemResource(item_resource_type.clone(), cost + 1)
            }
        }
    ];
    modifier_gain
}

fn execute_craft_reroll_modifier_costs(rng: &mut ThreadRng) -> (Vec<ModifierCost>, u64) {
    let mut modifier_costs = Vec::new();
    let mut cost = 0;
    if rng.gen_range(0..2) != 0 {
        cost = rng.gen_range(1..10);
        modifier_costs.push(ModifierCost::FlatItemResource(ItemResourceType::Mana, cost));
    }
    (modifier_costs, cost)
}


#[cfg(test)]
mod tests_int {
    use crate::command_craft_reroll_modifier::{execute_craft_reroll_modifier};
    use crate::command_equip_unequip::execute_swap_equipped_item;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_craft_item() {
        let mut game = generate_testing_game();

        assert_eq!(Err("Cant pay the crafting cost for reroll_modifier, the cost is 5 and you only have Some(0)".to_string()), execute_craft_reroll_modifier(&mut game, 0, 0));

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);

        let old_item = game.inventory[0].clone();

        let result = execute_craft_reroll_modifier(&mut game, 0, 0);

        assert!(result.is_ok());
        assert_ne!(old_item, result.unwrap());

        let old_item = game.inventory[0].clone();
        let old_gold = game.treasure.get(&Gold).unwrap().clone();

        assert_eq!(Err("inventory_index 99 is not within the range of the inventory 9".to_string()), execute_craft_reroll_modifier(&mut game, 99, 0));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(old_gold, *game.treasure.get(&Gold).unwrap());

        assert_eq!(Err("modifier_index 0 is not within the range of the item modifiers 1".to_string()), execute_craft_reroll_modifier(&mut game, 0, 99));

        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(old_gold, *game.treasure.get(&Gold).unwrap());
    }
}