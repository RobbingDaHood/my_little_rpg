#[cfg(test)]
mod tests_int {
    use crate::command::expand_places::execute;
    use crate::command::r#move::execute as execute_move_command;
    use crate::generator::game::new_testing;
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_places() {
        let mut game = new_testing(Some([1; 16]));
        assert_eq!(10, game.places.len());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "Cant pay the crafting cost, the cost is {Gold: 100} and you only have {}"
                    .to_string()
            )),
            execute(&mut game)
        );

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(10, game.places.len());

        let result = execute(&mut game);

        assert!(result.is_ok());
        assert_eq!(11, game.places.len());
    }

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute(&mut game);

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute(&mut game);
            assert_eq!(original_result, result);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 999_999);

        for _i in 1..438 {
            assert!(execute(&mut game).is_ok());
        }
    }
}
