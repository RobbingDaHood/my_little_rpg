#[cfg(test)]
mod tests_int {
    use crate::{
        command::{expand_elements::execute, r#move::execute as execute_move_command},
        generator::game::new,
        my_little_rpg_errors::MyError,
        the_world::treasure_types::TreasureType::Gold,
    };

    #[test]
    fn test_execute_expand_elements() {
        let mut game = new(Some([1; 16]));
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "Cant pay the crafting cost, the cost is {Gold: 10} and you only have {}"
                    .to_string()
            )),
            execute(&mut game)
        );

        for _i in 0..1000 {
            assert!(execute_move_command(&mut game, 0).is_ok());
        }
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(1, game.difficulty.max_resistance.len());
        assert_eq!(1, game.difficulty.min_resistance.len());

        for i in 2..10 {
            let result = execute(&mut game);

            assert!(result.is_ok());
            assert_eq!(i, game.difficulty.max_resistance.len());
            assert_eq!(i, game.difficulty.min_resistance.len());
        }

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "Already at maximum elements.".to_string()
            )),
            execute(&mut game)
        );
        assert_eq!(9, game.difficulty.max_resistance.len());
        assert_eq!(9, game.difficulty.min_resistance.len());
    }

    #[test]
    fn seeding_test() {
        let mut game = new(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute(&mut game);

        for _i in 1..1000 {
            let mut game = new(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute(&mut game);
            assert_eq!(original_result, result);
        }
    }
}
