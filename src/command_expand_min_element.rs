use rand::Rng;
use crate::attack_types::AttackType;
use crate::Game;
use crate::treasure_types::TreasureType::Gold;

pub fn execute_expand_min_element(game: &mut Game) -> Result<AttackType, String> {
    //Crafting cost
    let crafting_cost = game.place_generator_input.min_resistance.values().sum::<u64>() / game.place_generator_input.min_resistance.len() as u64;

    let max_possible_elements : Vec<AttackType> = game.place_generator_input.min_resistance.iter()
        .filter(|(attack_type, amount)| game.place_generator_input.max_resistance.get(attack_type).unwrap() > &(*amount + crafting_cost))
        .map(|(attack_type, _)| attack_type.clone())
        .collect();

    if max_possible_elements.len() < 1 {
        return Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string());
    }

    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_min_element, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Increase min of existing element
    let mut rng = rand::thread_rng();
    let picked_element = rng.gen_range(0..max_possible_elements.len());
    let picked_element = max_possible_elements[picked_element].clone();

    *game.place_generator_input.min_resistance.get_mut(&picked_element).unwrap() += crafting_cost;

    Ok(picked_element.clone())
}

#[cfg(test)]
mod tests_int {
    use crate::attack_types::AttackType;
    use crate::command_expand_max_element::execute_expand_max_element;
    use crate::command_expand_min_element::execute_expand_min_element;
    use crate::command_move::execute_move_command;
    use crate::game_generator::{generate_new_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_min_element() {
        let mut game = generate_new_game();
        assert_eq!(1, game.place_generator_input.min_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());

        assert_eq!(Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()), execute_expand_min_element(&mut game));

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }

        assert_eq!(Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()), execute_expand_min_element(&mut game));
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.place_generator_input.min_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());

        assert!(execute_expand_max_element(&mut game).is_ok());

        assert_eq!(Ok(AttackType::Physical), execute_expand_min_element(&mut game));
        assert_eq!(Err("There are no element minimum values that can be upgraded, consider expanding a max element value.".to_string()), execute_expand_min_element(&mut game));
    }
}