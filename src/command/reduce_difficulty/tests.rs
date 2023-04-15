#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;

    use crate::{
        command::reduce_difficulty::execute,
        generator::game::new_testing,
        the_world::{
            attack_types::DamageType, difficulty::Difficulty, treasure_types::TreasureType::Gold,
        },
        Game,
    };

    #[test]
    fn test_execute_reduce_difficulty() {
        let mut game = new_testing(Some([1; 16]));

        game.difficulty = Difficulty {
            min_resistance: HashMap::from([(DamageType::Physical, 10), (DamageType::Fire, 10)]),
            max_resistance: HashMap::from([(DamageType::Physical, 11), (DamageType::Fire, 11)]),
            min_simultaneous_resistances: 15,
            max_simultaneous_resistances: 7,
        };

        let _ = execute(&mut game);

        assert_eq!(15, game.difficulty.min_simultaneous_resistances);
        assert_eq!(7, game.difficulty.max_simultaneous_resistances);

        assert_eq!(
            10,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Physical)
                .unwrap()
        );
        assert_eq!(
            2,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(
            11,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Physical)
                .unwrap()
        );
        assert_eq!(
            5,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        let _ = execute(&mut game);

        assert_eq!(15, game.difficulty.min_simultaneous_resistances);
        assert_eq!(7, game.difficulty.max_simultaneous_resistances);

        assert_eq!(
            2,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Physical)
                .unwrap()
        );
        assert_eq!(
            2,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(
            5,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Physical)
                .unwrap()
        );
        assert_eq!(
            5,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        let _ = execute(&mut game);

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(
            None,
            game.difficulty.min_resistance.get(&DamageType::Physical)
        );
        assert_eq!(
            2,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(
            None,
            game.difficulty.max_resistance.get(&DamageType::Physical)
        );
        assert_eq!(
            5,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        let _ = execute(&mut game);

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(
            None,
            game.difficulty.min_resistance.get(&DamageType::Physical)
        );
        assert_eq!(
            1,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(
            None,
            game.difficulty.max_resistance.get(&DamageType::Physical)
        );
        assert_eq!(
            2,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(1, count_places_possible_rolls_equal_difficulty(&game));

        let _ = execute(&mut game);

        assert_eq!(1, game.difficulty.min_simultaneous_resistances);
        assert_eq!(1, game.difficulty.max_simultaneous_resistances);

        assert_eq!(
            None,
            game.difficulty.min_resistance.get(&DamageType::Physical)
        );
        assert_eq!(
            1,
            *game
                .difficulty
                .min_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(
            None,
            game.difficulty.max_resistance.get(&DamageType::Physical)
        );
        assert_eq!(
            2,
            *game
                .difficulty
                .max_resistance
                .get(&DamageType::Fire)
                .unwrap()
        );

        assert_eq!(
            2,
            game.places
                .iter()
                .map(|place| place.item_reward_possible_rolls.clone())
                .filter(|roll| roll.eq(&game.difficulty))
                .count()
        );
    }

    fn count_places_possible_rolls_equal_difficulty(game: &Game) -> usize {
        game.places
            .iter()
            .map(|place| place.item_reward_possible_rolls.clone())
            .filter(|roll| roll.eq(&game.difficulty))
            .count()
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
}
