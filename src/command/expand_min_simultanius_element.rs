use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandMaxSimultaneousElementReport {
    new_min_simultaneous_resistances: u8,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_min_simultaneous_element_json(game: &mut Game) -> Value {
    match execute_expand_min_simultaneous_element(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result)
    }
}

pub fn execute_expand_min_simultaneous_element(game: &mut Game) -> Result<ExecuteExpandMaxSimultaneousElementReport, MyError> {
    if game.difficulty.min_simultaneous_resistances >= game.difficulty.max_simultaneous_resistances {
        return Err(MyError::create_execute_command_error(format!("execute_expand_min_simultaneous_element {} is already equal to max_simultaneous_resistances {}. Consider calling ExpandMaxSimultaneousElement.", game.difficulty.max_simultaneous_resistances, game.difficulty.max_resistance.len())));
    }

    //Crafting cost
    let crafting_cost = execute_expand_min_simultaneous_element_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Increase max of existing element
    game.difficulty.min_simultaneous_resistances += 1;

    Ok(ExecuteExpandMaxSimultaneousElementReport {
        new_min_simultaneous_resistances: game.difficulty.min_simultaneous_resistances,
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_min_simultaneous_element_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_min_simultaneous_element_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, u64::from(game.difficulty.min_simultaneous_resistances) * 10)])
}

#[cfg(test)]
mod tests_int {
    use crate::command::expand_elements::execute as execute_expand_elements;
    use crate::command::expand_max_simultaneous_element::execute as execute_expand_max_simultaneous_element;
    use crate::command::expand_min_simultanius_element::execute_expand_min_simultaneous_element;
    use crate::generator::game::new;
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_min_simultaneous_element() {
        let mut game = new(Some([1; 16]));
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);
        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(Err(MyError::create_execute_command_error("execute_expand_min_simultaneous_element 1 is already equal to max_simultaneous_resistances 1. Consider calling ExpandMaxSimultaneousElement.".to_string())), execute_expand_min_simultaneous_element(&mut game));

        game.treasure.insert(Gold, 20);
        assert!(execute_expand_elements(&mut game).is_ok());
        assert!(execute_expand_max_simultaneous_element(&mut game).is_ok());

        assert_eq!(Err(MyError::create_execute_command_error("Cant pay the crafting cost, the cost is {Gold: 10} and you only have {Gold: 0}".to_string())), execute_expand_min_simultaneous_element(&mut game));

        *game.treasure.get_mut(&Gold).unwrap() += 1000;
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(2, game.difficulty.max_resistance.len());
        assert_eq!(2, game.difficulty.max_simultaneous_resistances);
        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(2, game.difficulty.min_resistance.len());

        let result = execute_expand_min_simultaneous_element(&mut game);
        assert!(result.is_ok());
        assert_eq!(2, result.as_ref().unwrap().new_min_simultaneous_resistances);
        assert_eq!(10, *result.as_ref().unwrap().paid_cost.get(&Gold).unwrap());
        assert_eq!(2, game.difficulty.max_resistance.len());
        assert_eq!(2, game.difficulty.max_simultaneous_resistances);
        assert_eq!(2, game.difficulty.min_simultaneous_resistances);
        assert_eq!(2, game.difficulty.min_resistance.len());

        assert_eq!(Err(MyError::create_execute_command_error("execute_expand_min_simultaneous_element 2 is already equal to max_simultaneous_resistances 2. Consider calling ExpandMaxSimultaneousElement.".to_string())), execute_expand_min_simultaneous_element(&mut game));
        assert_eq!(2, game.difficulty.max_resistance.len());
        assert_eq!(2, game.difficulty.min_simultaneous_resistances);
        assert_eq!(2, game.difficulty.min_resistance.len());
    }
}