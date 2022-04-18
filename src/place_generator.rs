use std::collections::{HashMap, HashSet};
use crate::attack_types;
use crate::place::Place;
use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::treasure_types::TreasureType;
use rand::Rng;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PlaceGeneratorInput {
    pub(crate) max_resistance: HashMap<attack_types::AttackType, u64>,
    pub(crate) min_resistance: HashMap<attack_types::AttackType, u64>,
}

pub fn generate_place(input: &PlaceGeneratorInput) -> Place {
    let mut resistance = HashMap::new();
    let mut reward = HashMap::new();

    let mut relevant_attack_types = HashSet::new();

    for attack_type in input.max_resistance.keys().chain(input.min_resistance.keys()).collect::<Vec<&AttackType>>() {
        relevant_attack_types.insert(attack_type);
    }

    let mut rng = rand::thread_rng();
    let mut resistance_sum = 0;
    for attack_type in relevant_attack_types {
        let max_value = input.max_resistance.get(attack_type);
        let min_value = input.min_resistance.get(attack_type);

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


            let resistance_value = rng.gen_range(*min_value..*max_value);
            resistance.insert(attack_type.clone(), resistance_value);
            resistance_sum += resistance_value;
        }
    }

    reward.insert(TreasureType::Gold, resistance_sum);

    Place { resistance, reward }
}


#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;
    use crate::attack_types::AttackType;
    use crate::treasure_types::TreasureType;
    use crate::place_generator::{generate_place, PlaceGeneratorInput};

    #[test]
    fn test_generate_place() {
        let mut min = HashMap::new();
        min.insert(AttackType::Physical, 1);
        min.insert(AttackType::Fire, 2);
        min.insert(AttackType::Frost, 3);
        min.insert(AttackType::Lightning, 4);
        min.insert(AttackType::Light, 5);
        min.insert(AttackType::Darkness, 6);
        min.insert(AttackType::Nature, 7);
        min.insert(AttackType::Corruption, 8);

        let mut max = HashMap::new();
        max.insert(AttackType::Fire, 20);
        max.insert(AttackType::Frost, 30);
        max.insert(AttackType::Lightning, 40);
        max.insert(AttackType::Light, 50);
        max.insert(AttackType::Darkness, 60);
        max.insert(AttackType::Nature, 70);
        max.insert(AttackType::Corruption, 80);
        max.insert(AttackType::Holy, 90);

        let input = PlaceGeneratorInput {
            min_resistance: min,
            max_resistance: max,
        };

        let place = generate_place(&input);

        println!("test_generate_place: {:?}", place);
    }

    #[test]
    fn test_generate_place_one_element() {
        let min = HashMap::new();

        let mut max = HashMap::new();
        max.insert(AttackType::Fire, 2);
        let input = PlaceGeneratorInput {
            min_resistance: min,
            max_resistance: max,
        };

        let place = generate_place(&input);

        assert_eq!(&1, place.resistance.get(&AttackType::Fire).unwrap());
        assert_eq!(&1, place.reward.get(&TreasureType::Gold).unwrap())
    }
}