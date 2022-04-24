use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use crate::{attack_types, Game};
use crate::place::Place;
use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::treasure_types::TreasureType;
use rand::Rng;

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

    let mut resistance_sum = 0;
    let mut count_elements = 0;
    while count_elements < minimum_elements {
        for attack_type in relevant_attack_types.clone() {
            if minimum_elements == relevant_attack_types.len() || game.random_generator_state.gen_range(0..2) != 0 {
                let max_value = game.difficulty.max_resistance.get(attack_type);
                let min_value = game.difficulty.min_resistance.get(attack_type);

                if max_value.is_none() && min_value.is_none() {
                    println!("Error: generate_place: Could not find min nor max values for type {:?}. Will not add the attack_type to resistances.", attack_type);
                } else if max_value.is_none() {
                    println!("Error: generate_place: Could not find max even though min were defined {:?}. Will not add the attack_type to resistances.", attack_type);
                } else {
                    let max_value = max_value.unwrap();
                    let min_value = min_value.unwrap_or(&1);

                    if max_value <= min_value {
                        println!("Error: generate_place: Min {} is above max {}, for {:?}. Will not add the attack_type to resistances.", min_value, max_value, attack_type);
                    }

                    let resistance_value = game.random_generator_state.gen_range(*min_value..*max_value);
                    resistance.insert(attack_type.clone(), resistance_value);
                    resistance_sum += resistance_value;
                }
                count_elements += 1;
            }

            if count_elements >= maximum_elements {
                break;
            }
        }
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

        println!("test_generate_place: {:?}", place);
        assert_eq!(place.resistance.len(), 8);
    }

    #[test]
    fn test_generate_place_one_element() {
        let mut game = generate_new_game(Some([1; 16]));

        let place = generate_place(&mut game);

        assert_eq!(&1, place.resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(&3, place.reward.get(&TreasureType::Gold).unwrap());
        assert_eq!(place.resistance.len(), 1);
    }
}