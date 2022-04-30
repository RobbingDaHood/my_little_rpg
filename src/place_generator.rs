use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use crate::{attack_types, Game};
use crate::place::Place;
use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::treasure_types::TreasureType;
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Difficulty {
    pub(crate) max_resistance: HashMap<attack_types::AttackType, u64>,
    pub(crate) min_resistance: HashMap<attack_types::AttackType, u64>,
    pub(crate) max_simultaneous_resistances: u8,
    pub(crate) min_simultaneous_resistances: u8,
}

pub fn generate_place(game: &mut Game) -> Place {
    let mut resistance: HashMap<AttackType, u64> = HashMap::new();
    let mut reward = HashMap::new();

    let mut relevant_attack_types = HashSet::new();

    for attack_type in game.difficulty.max_resistance.keys().chain(game.difficulty.min_resistance.keys()).collect::<Vec<&AttackType>>() {
        relevant_attack_types.insert(attack_type);
    }

    let minimum_elements = min(relevant_attack_types.len(), game.difficulty.min_simultaneous_resistances as usize);
    let maximum_elements = min(relevant_attack_types.len(), game.difficulty.max_simultaneous_resistances as usize);
    let resistance_numbers = game.random_generator_state.gen_range(minimum_elements..maximum_elements.add(1));

    let attack_types = AttackType::get_all().iter()
        .filter(|attack_type| game.difficulty.min_resistance.contains_key(attack_type))
        .map(|attack_type| attack_type.clone())
        .collect::<Vec<AttackType>>();

    let mut resistance_sum = 0;
    let mut count_elements = 0;
    while count_elements < resistance_numbers  {
        let attack_type = attack_types.choose(&mut game.random_generator_state).unwrap();

        let min_value = game.difficulty.min_resistance.get(attack_type).unwrap();
        let max_value = game.difficulty.max_resistance.get(attack_type).unwrap();

        let resistance_value = game.random_generator_state.gen_range(*min_value..*max_value);
        resistance.insert(attack_type.clone(), resistance_value);
        resistance_sum += resistance_value;

        count_elements += 1;
    }

    let reward_from_resistance = (resistance_sum / AttackType::get_all().len() as u64) * count_elements as u64;

    let possible_resistance_values_sum = game.difficulty.max_resistance.values().chain(game.difficulty.min_resistance.values()).sum::<u64>();
    let average_possible_resistance_values = possible_resistance_values_sum / relevant_attack_types.len() as u64;
    let reward_from_difficulty = average_possible_resistance_values / max(game.places.len(), 1) as u64;

    reward.insert(TreasureType::Gold, reward_from_resistance + reward_from_difficulty);

    Place { resistance, reward }
}


#[cfg(test)]
mod tests_int {
    use crate::attack_types::AttackType;
    use crate::game_generator::{generate_new_game, generate_testing_game};
    use crate::treasure_types::TreasureType;
    use crate::place_generator::{generate_place};

    #[test]
    fn test_generate_place() {
        let mut game = generate_testing_game(Some([1; 16]));

        let place = generate_place(&mut game);

        assert_eq!(place.resistance.len(), 5);
    }

    #[test]
    fn test_generate_place_one_element() {
        let mut game = generate_new_game(Some([1; 16]));

        let place = generate_place(&mut game);

        assert_eq!(&1, place.resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(&3, place.reward.get(&TreasureType::Gold).unwrap());
        assert_eq!(place.resistance.len(), 1);
    }

    #[test]
    fn seeding_test() {
        let original_game = generate_testing_game(Some([1; 16]));

        for _i in 1..1000 {
            let game = generate_testing_game(Some([1; 16]));
            assert_eq!(original_game, game);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = generate_testing_game(Some([1; 16]));

        for _i in 1..438 {
            generate_place(&mut game);
        }
    }
}