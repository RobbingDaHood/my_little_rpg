#[cfg(test)]
mod tests_int {
    use crate::{
        command::{
            expand_max_element::execute as execute_expand_max_element,
            expand_min_element::execute as execute_expand_min_element, r#move::execute,
        },
        generator::game::{new, new_testing},
        my_little_rpg_errors::MyError,
        the_world::treasure_types::TreasureType::Gold,
    };

    #[test]
    fn test_execute_expand_min_element() {
        let mut game = new(Some([1; 16]));
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "Cant pay the crafting cost, the cost is {Gold: 1} and you only have {}"
                    .to_string()
            )),
            execute_expand_min_element(&mut game)
        );

        for _i in 0..1000 {
            assert!(execute(&mut game, 0).is_ok());
        }

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "There are no element minimum values that can be upgraded, consider expanding a \
                 max element value."
                    .to_string()
            )),
            execute_expand_min_element(&mut game)
        );
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.difficulty.min_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert!(execute_expand_max_element(&mut game).is_ok());

        assert!(execute_expand_min_element(&mut game).is_ok());
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "There are no element minimum values that can be upgraded, consider expanding a \
                 max element value."
                    .to_string()
            )),
            execute_expand_min_element(&mut game)
        );
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

        let number_of_unchanged_elements = original_difficulty
            .min_resistance
            .iter()
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
