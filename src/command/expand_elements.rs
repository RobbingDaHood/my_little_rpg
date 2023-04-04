use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Game;
use crate::the_world::attack_types::AttackType;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandElementsReport {
    new_element_type: AttackType,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_elements(game: &mut Game) -> Result<ExecuteExpandElementsReport, String> {
    if game.difficulty.max_resistance.len() >= AttackType::get_all().len() {
        return Err("Already at maximum elements.".to_string());
    }

    //Crafting cost
    let crafting_cost = execute_expand_elements_calculate_cost(game);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message);
    };

    //Add new element
    let new_element = AttackType::get_all()[game.difficulty.max_resistance.len()].clone();
    game.difficulty.max_resistance.insert(new_element.clone(), 2);
    game.difficulty.min_resistance.insert(new_element.clone(), 1);

    Ok(ExecuteExpandElementsReport {
        new_element_type: new_element,
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_elements_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_elements_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.difficulty.max_resistance.len() * 10) as u64)])
}

#[cfg(test)]
mod tests_int {
    use crate::command::expand_elements::execute_expand_elements;
    use crate::command::r#move::execute_move_command;
    use crate::generator::game_generator::generate_new_game;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_elements() {
        let mut game = generate_new_game(Some([1; 16]));
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 10} and you only have {}".to_string()), execute_expand_elements(&mut game));

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        for i in 2..10 {
            let result = execute_expand_elements(&mut game);

            assert!(result.is_ok());
            assert_eq!(i, game.difficulty.max_resistance.len());
            assert_eq!(i, game.difficulty.min_resistance.len());
        }

        assert_eq!(Err("Already at maximum elements.".to_string()), execute_expand_elements(&mut game));
        assert_eq!(9, game.difficulty.max_resistance.len());
        assert_eq!(9, game.difficulty.min_resistance.len());
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_new_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_expand_elements(&mut game);

        for _i in 1..1000 {
            let mut game = generate_new_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_expand_elements(&mut game);
            assert_eq!(original_result, result);
        }
    }
}