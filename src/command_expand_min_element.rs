use std::collections::HashMap;
use rand::Rng;
use crate::attack_types::AttackType;
use crate::Game;
use crate::place_generator::Difficulty;
use crate::treasure_types::{pay_crafting_cost, TreasureType};
use crate::treasure_types::TreasureType::Gold;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMinElementReport {
    new_place_generator_input: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_min_element(game: &mut Game) -> Result<ExecuteExpandMinElementReport, String> {
    //Crafting cost
    let crafting_cost = execute_expand_min_element_calculate_cost(game);
    let crafting_gold_cost = crafting_cost.get(&Gold).unwrap();

    let max_possible_elements: Vec<AttackType> = game.difficulty.min_resistance.iter()
        .filter(|(attack_type, amount)| game.difficulty.max_resistance.get(attack_type).unwrap() > &(*amount + crafting_gold_cost))
        .map(|(attack_type, _)| attack_type.clone())
        .collect();

    if max_possible_elements.len() < 1 {
        return Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string());
    }

    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message)
    };

    //Increase min of existing element
    let mut rng = rand::thread_rng();
    let picked_element = rng.gen_range(0..max_possible_elements.len());
    let picked_element = max_possible_elements[picked_element].clone();

    *game.difficulty.min_resistance.get_mut(&picked_element).unwrap() += crafting_gold_cost;

    Ok(ExecuteExpandMinElementReport {
        new_place_generator_input: game.difficulty.clone(),
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
    use crate::command_expand_max_element::execute_expand_max_element;
    use crate::command_expand_min_element::execute_expand_min_element;
    use crate::command_move::execute_move_command;
    use crate::game_generator::{generate_new_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_min_element() {
        let mut game = generate_new_game();
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert_eq!(Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()), execute_expand_min_element(&mut game));

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }

        assert_eq!(Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()), execute_expand_min_element(&mut game));
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert!(execute_expand_max_element(&mut game).is_ok());

        assert!(execute_expand_min_element(&mut game).is_ok());
        assert_eq!(Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()), execute_expand_min_element(&mut game));
    }
}