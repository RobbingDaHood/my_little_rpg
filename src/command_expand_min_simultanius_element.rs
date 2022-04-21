use std::collections::HashMap;
use crate::Game;
use crate::treasure_types::TreasureType;
use crate::treasure_types::TreasureType::Gold;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMaxSimultaneousElementReport {
    new_min_simultaneous_resistances: u8,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_min_simultaneous_element(game: &mut Game) -> Result<ExecuteExpandMaxSimultaneousElementReport, String> {
    if game.place_generator_input.min_simultaneous_resistances >= game.place_generator_input.max_simultaneous_resistances {
        return Err(format!("execute_expand_min_simultaneous_element {} is already equal to max_simultaneous_resistances {}. Consider calling ExpandMaxSimultaneousElement.", game.place_generator_input.max_simultaneous_resistances, game.place_generator_input.max_resistance.len()));
    }

    //Crafting cost
    let crafting_cost = execute_expand_min_simultaneous_element_calculate_cost(game);
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_min_simultaneous_element, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Increase max of existing element
    game.place_generator_input.min_simultaneous_resistances += 1;

    Ok(ExecuteExpandMaxSimultaneousElementReport {
        new_min_simultaneous_resistances: game.place_generator_input.min_simultaneous_resistances,
        paid_cost: HashMap::from([(Gold, crafting_cost.clone())]),
        new_cost: HashMap::from([(Gold, execute_expand_min_simultaneous_element_calculate_cost(game))]),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_min_simultaneous_element_calculate_cost(game: &mut Game) -> u64 {
    game.place_generator_input.min_simultaneous_resistances as u64 * 10
}

#[cfg(test)]
mod tests_int {
    use crate::command_expand_elements::execute_expand_elements;
    use crate::command_expand_max_simultaneous_element::execute_expand_max_simultaneous_element;
    use crate::command_expand_min_simultanius_element::execute_expand_min_simultaneous_element;
    use crate::game_generator::{generate_new_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_min_simultaneous_element() {
        let mut game = generate_new_game();
        assert_eq!(1, game.place_generator_input.max_resistance.len());
        assert_eq!(1, game.place_generator_input.max_simultaneous_resistances);
        assert_eq!(1, game.place_generator_input.min_simultaneous_resistances);
        assert_eq!(1, game.place_generator_input.min_resistance.len());
        assert_eq!(Err("execute_expand_min_simultaneous_element 1 is already equal to max_simultaneous_resistances 1. Consider calling ExpandMaxSimultaneousElement.".to_string()), execute_expand_min_simultaneous_element(&mut game));

        game.treasure.insert(Gold, 20);
        assert!(execute_expand_elements(&mut game).is_ok());
        assert!(execute_expand_max_simultaneous_element(&mut game).is_ok());

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_min_simultaneous_element, the cost is 10 and you only have Some(0)".to_string()), execute_expand_min_simultaneous_element(&mut game));

        *game.treasure.get_mut(&Gold).unwrap() += 1000;
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(2, game.place_generator_input.max_resistance.len());
        assert_eq!(2, game.place_generator_input.max_simultaneous_resistances);
        assert_eq!(1, game.place_generator_input.min_simultaneous_resistances);
        assert_eq!(2, game.place_generator_input.min_resistance.len());

        let result = execute_expand_min_simultaneous_element(&mut game);
        assert!(result.is_ok());
        assert_eq!(2, result.as_ref().unwrap().new_min_simultaneous_resistances);
        assert_eq!(10, *result.as_ref().unwrap().paid_cost.get(&Gold).unwrap());
        assert_eq!(2, game.place_generator_input.max_resistance.len());
        assert_eq!(2, game.place_generator_input.max_simultaneous_resistances);
        assert_eq!(2, game.place_generator_input.min_simultaneous_resistances);
        assert_eq!(2, game.place_generator_input.min_resistance.len());

        assert_eq!(Err("execute_expand_min_simultaneous_element 2 is already equal to max_simultaneous_resistances 2. Consider calling ExpandMaxSimultaneousElement.".to_string()), execute_expand_min_simultaneous_element(&mut game));
        assert_eq!(2, game.place_generator_input.max_resistance.len());
        assert_eq!(2, game.place_generator_input.min_simultaneous_resistances);
        assert_eq!(2, game.place_generator_input.min_resistance.len());
    }
}