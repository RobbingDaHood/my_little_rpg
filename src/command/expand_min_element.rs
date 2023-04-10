use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::attack_types::AttackType;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMinElementReport {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result)
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandMinElementReport, MyError> {
    //Crafting cost
    let crafting_cost = execute_expand_min_element_calculate_cost(game);
    let crafting_gold_cost = crafting_cost.get(&Gold).unwrap();

    let max_possible_elements: Vec<AttackType> = game.difficulty.min_resistance.iter()
        .filter(|(attack_type, amount)| game.difficulty.max_resistance.get(attack_type).unwrap() > &(*amount + crafting_gold_cost))
        .map(|(attack_type, _)| attack_type.clone())
        .collect();

    if max_possible_elements.is_empty() {
        return Err(MyError::create_execute_command_error("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()));
    }

    pay_crafting_cost(game, &crafting_cost)?;

    //Increase min of existing element
    let picked_element = game.random_generator_state.gen_range(0..max_possible_elements.len());
    let picked_element = max_possible_elements[picked_element].clone();

    *game.difficulty.min_resistance.get_mut(&picked_element).unwrap() += crafting_gold_cost;

    Ok(ExecuteExpandMinElementReport {
        new_difficulty: game.difficulty.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_min_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_min_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, game.difficulty.min_resistance.values().sum::<u64>() / game.difficulty.min_resistance.len() as u64)])
}

#[cfg(test)]
mod tests_int {
    use crate::command::expand_max_element::execute as execute_expand_max_element;
    use crate::command::expand_min_element::execute as execute_expand_min_element;
    use crate::command::r#move::execute;
    use crate::generator::game::{new, new_testing};
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_min_element() {
        let mut game = new(Some([1; 16]));
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert_eq!(Err(MyError::create_execute_command_error("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string())), execute_expand_min_element(&mut game));

        for _i in 0..1000 {
            assert!(execute(&mut game, 0).is_ok());
        }

        assert_eq!(Err(MyError::create_execute_command_error("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string())), execute_expand_min_element(&mut game));
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert!(execute_expand_max_element(&mut game).is_ok());

        assert!(execute_expand_min_element(&mut game).is_ok());
        assert_eq!(Err(MyError::create_execute_command_error("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string())), execute_expand_min_element(&mut game));
    }


    #[test]
    fn test_that_all_elements_can_be_hit() {
        let mut game = new_testing(Some([1; 16]));
        let original_difficulty = game.difficulty.clone();
        game.treasure.insert(Gold, 9_999_999);

        for _i in 0..65 {
            assert!(execute_expand_max_element(&mut game).is_ok());
            assert!(execute_expand_min_element(&mut game).is_ok());
        }

        let number_of_unchanged_elements = original_difficulty.min_resistance.iter()
            .filter(|(x, y)| game.difficulty.min_resistance.get(x).unwrap() == *y)
            .count();
        assert_eq!(0, number_of_unchanged_elements);
    }

    #[test]
    fn seeding_test() {
        let mut game = new(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        assert!(execute_expand_max_element(&mut game).is_ok());
        let original_result = execute_expand_min_element(&mut game);

        for _i in 1..1000 {
            let mut game = new(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            assert!(execute_expand_max_element(&mut game).is_ok());
            let result = execute_expand_min_element(&mut game);
            assert_eq!(original_result, result);
        }
    }
}