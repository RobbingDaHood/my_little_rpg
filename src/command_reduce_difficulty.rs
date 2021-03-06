use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Div;
use crate::Game;
use crate::treasure_types::{TreasureType};
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use rand::prelude::SliceRandom;
use crate::difficulty::Difficulty;
use crate::game::get_random_attack_type_from_unlocked;
use crate::place_generator::{generate_place};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ReduceDifficultyReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_reduce_difficulty(game: &mut Game) -> Result<ReduceDifficultyReport, String> {
    //Add new element
    let attack_type = get_random_attack_type_from_unlocked(&mut game.random_generator_state, &game.difficulty.min_resistance);

    let max_value = game.difficulty.max_resistance.get(&attack_type).unwrap();
    let new_max_value = max_value.div(2);

    if new_max_value < 4 && game.difficulty.max_resistance.len() > 1 {
        game.difficulty.max_resistance.remove(&attack_type);
        game.difficulty.min_resistance.remove(&attack_type);

        let max_simultaneous_resistances = usize::from(game.difficulty.max_simultaneous_resistances);
        if max_simultaneous_resistances > game.difficulty.max_resistance.len() {
            game.difficulty.max_simultaneous_resistances = min(game.difficulty.max_simultaneous_resistances, game.difficulty.max_resistance.len().try_into().unwrap());
        }

        if game.difficulty.min_simultaneous_resistances > game.difficulty.max_simultaneous_resistances {
            game.difficulty.min_simultaneous_resistances = game.difficulty.max_simultaneous_resistances;
        }
    } else {
        *game.difficulty.max_resistance.get_mut(&attack_type).unwrap() = max(2, new_max_value);
        let new_min = min(max(1, new_max_value.div(2)), *game.difficulty.min_resistance.get(&attack_type).unwrap());
        *game.difficulty.min_resistance.get_mut(&attack_type).unwrap() = min(max(1, new_max_value.div(2)), new_min);
    }

    let new_place = generate_place(game);
    *game.places.choose_mut(&mut game.random_generator_state).unwrap() = new_place;

    Ok(ReduceDifficultyReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: execute_execute_reduce_difficulty_cost(),
        new_cost: execute_execute_reduce_difficulty_cost(),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_execute_reduce_difficulty_cost() -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, 0)])
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;
    use crate::attack_types::AttackType;
    use crate::command_reduce_difficulty::execute_reduce_difficulty;
    use crate::difficulty::Difficulty;
    use crate::game_generator::{generate_testing_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_reduce_difficulty() {
        let mut game = generate_testing_game(Some([1; 16]));

        game.difficulty = Difficulty {
            min_resistance: HashMap::from([
                (AttackType::Physical, 10),
                (AttackType::Fire, 10),
            ]),
            max_resistance: HashMap::from([
                (AttackType::Physical, 11),
                (AttackType::Fire, 11),
            ]),
            min_simultaneous_resistances: 15,
            max_simultaneous_resistances: 7,
        };

        assert!(execute_reduce_difficulty(&mut game).is_ok());

        assert_eq!(15, game.difficulty.min_simultaneous_resistances);
        assert_eq!(7, game.difficulty.max_simultaneous_resistances);

        assert_eq!(10, *game.difficulty.min_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(11, *game.difficulty.max_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, game.places.iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count());

        assert!(execute_reduce_difficulty(&mut game).is_ok());

        assert_eq!(15, game.difficulty.min_simultaneous_resistances);
        assert_eq!(7, game.difficulty.max_simultaneous_resistances);

        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, game.places.iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count());

        assert!(execute_reduce_difficulty(&mut game).is_ok());

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(None, game.difficulty.min_resistance.get(&AttackType::Physical));
        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(None, game.difficulty.max_resistance.get(&AttackType::Physical));
        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, game.places.iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count());

        assert!(execute_reduce_difficulty(&mut game).is_ok());

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(None, game.difficulty.min_resistance.get(&AttackType::Physical));
        assert_eq!(1, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(None, game.difficulty.max_resistance.get(&AttackType::Physical));
        assert_eq!(2, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, game.places.iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count());

        assert!(execute_reduce_difficulty(&mut game).is_ok());

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(None, game.difficulty.min_resistance.get(&AttackType::Physical));
        assert_eq!(1, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(None, game.difficulty.max_resistance.get(&AttackType::Physical));
        assert_eq!(2, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(2, game.places.iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count());
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_reduce_difficulty(&mut game);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_reduce_difficulty(&mut game);
            assert_eq!(original_result, result);
        }
    }
}