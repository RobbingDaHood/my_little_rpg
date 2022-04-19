use rand::Rng;
use crate::attack_types::AttackType;
use crate::Game;
use crate::treasure_types::TreasureType::Gold;

pub fn execute_expand_max_element(game: &mut Game) -> Result<AttackType, String> {
    //Crafting cost
    let crafting_cost = game.place_generator_input.max_resistance.values().sum::<u64>() / game.place_generator_input.max_resistance.len() as u64;
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_max_element, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Increase max of existing element
    let min_possible_element = 0;
    let max_possible_element = game.place_generator_input.max_resistance.len();
    let mut rng = rand::thread_rng();
    let picked_element = rng.gen_range(min_possible_element..max_possible_element);
    let picked_element = AttackType::get_all()[picked_element].clone();

    *game.place_generator_input.max_resistance.get_mut(&picked_element).unwrap() += crafting_cost;

    Ok(picked_element.clone())
}

#[cfg(test)]
mod tests_int {
    use crate::command_expand_max_element::execute_expand_max_element;
    use crate::command_move::execute_move_command;
    use crate::game_generator::{generate_new_game};
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_max_element() {
        let mut game = generate_new_game();
        assert_eq!(1, game.place_generator_input.max_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_max_element, the cost is 2 and you only have Some(0)".to_string()), execute_expand_max_element(&mut game));

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.place_generator_input.max_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());

        for _i in 2..10 {
            let result = execute_expand_max_element(&mut game);

            assert!(result.is_ok());
            assert_eq!(1, game.place_generator_input.max_resistance.len());
            assert_eq!(1, game.place_generator_input.min_resistance.len());
        }

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_max_element, the cost is 512 and you only have Some(490)".to_string()), execute_expand_max_element(&mut game));
        assert_eq!(1, game.place_generator_input.max_resistance.len());
        assert_eq!(1, game.place_generator_input.min_resistance.len());
    }
}