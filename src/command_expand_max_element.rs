use std::collections::HashMap;
use crate::Game;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::difficulty::Difficulty;
use crate::treasure_types::{pay_crafting_cost, TreasureType};
use crate::game::get_random_attack_type_from_unlocked;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMaxElementReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_max_element(game: &mut Game) -> Result<ExecuteExpandMaxElementReport, String> {
    //Crafting cost
    let crafting_cost = execute_expand_max_element_calculate_cost(game);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message);
    };

    //Increase max of existing element
    let picked_element = get_random_attack_type_from_unlocked(&mut game.random_generator_state, &game.difficulty.min_resistance);

    *game.difficulty.max_resistance.get_mut(&picked_element).unwrap() += crafting_cost.get(&Gold).unwrap();

    Ok(ExecuteExpandMaxElementReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_max_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_max_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, game.difficulty.max_resistance.values().sum::<u64>() / game.difficulty.max_resistance.len() as u64)])
}

#[cfg(test)]
mod tests_int {
    use crate::command_expand_max_element::execute_expand_max_element;
    use crate::command_move::execute_move_command;
    use crate::game_generator::{generate_new_game, generate_testing_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_max_element() {
        let mut game = generate_new_game(Some([1; 16]));
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 2} and you only have {}".to_string()), execute_expand_max_element(&mut game));

        for _i in 0..100 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        for _i in 2..9 {
            let result = execute_expand_max_element(&mut game);

            assert!(result.is_ok());
            assert_eq!(1, game.difficulty.max_resistance.len());
            assert_eq!(1, game.difficulty.min_resistance.len());
        }

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 256} and you only have {Gold: 46}".to_string()), execute_expand_max_element(&mut game));
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());
    }


    #[test]
    fn test_that_all_elements_can_be_hit() {
        let mut game = generate_testing_game(Some([1; 16]));
        let original_difficulty = game.difficulty.clone();
        game.treasure.insert(Gold, 999999);

        for _i in 0..65 {
            assert!(execute_expand_max_element(&mut game).is_ok());
        }

        let number_of_unchanged_elements = original_difficulty.max_resistance.iter()
            .filter(|(x, y)| game.difficulty.max_resistance.get(x).unwrap() == *y)
            .count();
        assert_eq!(0, number_of_unchanged_elements);
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_new_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_expand_max_element(&mut game);

        for _i in 1..1000 {
            let mut game = generate_new_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_expand_max_element(&mut game);
            assert_eq!(original_result, result);
        }
    }
}