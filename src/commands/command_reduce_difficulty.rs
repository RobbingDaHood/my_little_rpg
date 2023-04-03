use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Div;

use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::Game;
use crate::game::get_random_attack_type_from_unlocked;
use crate::place_generator::generate_place;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::treasure_types::TreasureType;
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ReduceDifficultyReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_reduce_difficulty(game: &mut Game) -> ReduceDifficultyReport {
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

    ReduceDifficultyReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: execute_execute_reduce_difficulty_cost(),
        new_cost: execute_execute_reduce_difficulty_cost(),
        leftover_spending_treasure: game.treasure.clone(),
    }
}

pub fn execute_execute_reduce_difficulty_cost() -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, 0)])
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;

    use crate::commands::command_reduce_difficulty::execute_reduce_difficulty;
    use crate::Game;
    use crate::game_generator::generate_testing_game;
    use crate::the_world::attack_types::AttackType;
    use crate::the_world::difficulty::Difficulty;
    use crate::the_world::treasure_types::TreasureType::Gold;

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

        execute_reduce_difficulty(&mut game);

        assert_eq!(15, game.difficulty.min_simultaneous_resistances);
        assert_eq!(7, game.difficulty.max_simultaneous_resistances);

        assert_eq!(10, *game.difficulty.min_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(11, *game.difficulty.max_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        execute_reduce_difficulty(&mut game);

        assert_eq!(15, game.difficulty.min_simultaneous_resistances);
        assert_eq!(7, game.difficulty.max_simultaneous_resistances);

        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        execute_reduce_difficulty(&mut game);

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(None, game.difficulty.min_resistance.get(&AttackType::Physical));
        assert_eq!(2, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(None, game.difficulty.max_resistance.get(&AttackType::Physical));
        assert_eq!(5, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        execute_reduce_difficulty(&mut game);

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(None, game.difficulty.min_resistance.get(&AttackType::Physical));
        assert_eq!(1, *game.difficulty.min_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(None, game.difficulty.max_resistance.get(&AttackType::Physical));
        assert_eq!(2, *game.difficulty.max_resistance.get(&AttackType::Fire).unwrap());

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        execute_reduce_difficulty(&mut game);

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

    fn count_places_possible_rolls_equal_difficulty(game: &Game) -> usize {
        game.places.iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count()
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