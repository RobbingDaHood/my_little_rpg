use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::treasure_types::{pay_crafting_cost, TreasureType};
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandElementsReport {
    new_element_type: AttackType,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_elements(game: &mut Game) -> Result<ExecuteExpandElementsReport, String> {
    if game.place_generator_input.max_resistance.len() >= AttackType::get_all().len() {
        return Err("Already at maximum elements.".to_string());
    }

    //Crafting cost
    let crafting_cost = execute_expand_elements_calculate_cost(game);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message)
    };

    //Add new element
    let new_element = AttackType::get_all()[game.place_generator_input.max_resistance.len()].clone();
    game.place_generator_input.max_resistance.insert(new_element.clone(), 2);
    game.place_generator_input.min_resistance.insert(new_element.clone(), 1);

    Ok(ExecuteExpandElementsReport {
        new_element_type: new_element.clone(),
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_elements_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_elements_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.place_generator_input.max_resistance.len() * 10) as u64)])
}

#[cfg(test)]
mod tests_int {
    use crate::command_expand_elements::execute_expand_elements;
    use crate::command_move::execute_move_command;
    use crate::game_generator::{generate_new_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_elements() {
        let mut game = generate_new_game();
        assert_eq!(1, game.place_generator_input.max_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 10} and you only have {}".to_string()), execute_expand_elements(&mut game));

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.place_generator_input.max_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());

        for i in 2..10 {
            let result = execute_expand_elements(&mut game);

            assert!(result.is_ok());
            assert_eq!(i, game.place_generator_input.max_resistance.len());
            assert_eq!(i, game.place_generator_input.min_resistance.len());
        }

        assert_eq!(Err("Already at maximum elements.".to_string()), execute_expand_elements(&mut game));
        assert_eq!(9, game.place_generator_input.max_resistance.len());
        assert_eq!(9, game.place_generator_input.min_resistance.len());
    }
}