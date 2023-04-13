use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use rand::{seq::SliceRandom, Rng};

use crate::{
    the_world::{attack_types::AttackType, place::Place, treasure_types::TreasureType},
    Game,
};

mod tests;

pub fn new(game: &mut Game) -> Place {
    let mut resistance: HashMap<AttackType, u64> = HashMap::new();
    let mut reward = HashMap::new();

    let mut relevant_attack_types = HashSet::new();

    for attack_type in game
        .difficulty
        .max_resistance
        .keys()
        .chain(game.difficulty.min_resistance.keys())
        .collect::<Vec<&AttackType>>()
    {
        relevant_attack_types.insert(attack_type);
    }

    let minimum_elements = min(
        relevant_attack_types.len(),
        game.difficulty.min_simultaneous_resistances as usize,
    );
    let maximum_elements = min(
        relevant_attack_types.len(),
        game.difficulty.max_simultaneous_resistances as usize,
    );
    let resistance_numbers = game
        .random_generator_state
        .gen_range(minimum_elements..=maximum_elements);

    let mut possible_attack_types: Vec<&AttackType> =
        game.difficulty.min_resistance.keys().collect();
    possible_attack_types.sort();

    let mut resistance_sum = 0;
    let mut count_elements = 0;
    while count_elements < resistance_numbers {
        let attack_type = *possible_attack_types
            .choose(&mut game.random_generator_state)
            .unwrap();

        let min_value = game.difficulty.min_resistance.get(attack_type).unwrap();
        let max_value = game.difficulty.max_resistance.get(attack_type).unwrap();

        let resistance_value = game
            .random_generator_state
            .gen_range(*min_value..=*max_value);
        resistance.insert(attack_type.clone(), resistance_value);
        resistance_sum += resistance_value;

        count_elements += 1;
    }

    let reward_from_resistance =
        (resistance_sum / AttackType::get_all().len() as u64) * count_elements as u64;

    let possible_resistance_values_sum = game
        .difficulty
        .max_resistance
        .values()
        .chain(game.difficulty.min_resistance.values())
        .sum::<u64>();
    let average_possible_resistance_values =
        possible_resistance_values_sum / relevant_attack_types.len() as u64;
    let reward_from_difficulty = max(
        1,
        average_possible_resistance_values / max(game.places.len(), 1) as u64,
    );

    reward.insert(
        TreasureType::Gold,
        reward_from_resistance + reward_from_difficulty,
    );

    Place {
        resistance,
        reward,
        item_reward_possible_rolls: game.difficulty.clone(),
    }
}
