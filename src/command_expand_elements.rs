use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::treasure_types::TreasureType;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandElementsReport {
    new_element_type: AttackType,
    cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_elements(game: &mut Game) -> Result<ExecuteExpandElementsReport, String> {
    if game.place_generator_input.max_resistance.len() >= AttackType::get_all().len() {
        return Err("Already at maximum elements.".to_string());
    }

    //Crafting cost
    let crafting_cost = (game.place_generator_input.max_resistance.len() * 10) as u64;
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_elements, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Add new element
    let new_element = AttackType::get_all()[game.place_generator_input.max_resistance.len()].clone();
    game.place_generator_input.max_resistance.insert(new_element.clone(), 2);
    game.place_generator_input.min_resistance.insert(new_element.clone(), 1);

    Ok(ExecuteExpandElementsReport {
        new_element_type: new_element.clone(),
        cost: HashMap::from([(Gold, crafting_cost.clone())]),
        leftover_spending_treasure: game.treasure.clone(),
    })
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

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_elements, the cost is 10 and you only have Some(0)".to_string()), execute_expand_elements(&mut game));

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