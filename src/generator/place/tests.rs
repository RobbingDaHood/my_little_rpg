#[cfg(test)]
mod tests_int {
    use crate::generator::game::{new as new_game, new_testing as new_game_testing};
    use crate::generator::place::new as new_place;
    use crate::the_world::attack_types::AttackType;
    use crate::the_world::treasure_types::TreasureType;

    #[test]
    fn test_generate_place() {
        let mut game = new_game_testing(Some([1; 16]));

        let place = new_place(&mut game);

        assert_eq!(place.resistance.len(), 6);
        assert_eq!(game.difficulty, place.item_reward_possible_rolls);
    }

    #[test]
    fn test_generate_place_one_element() {
        let mut game = new_game(Some([1; 16]));

        let place = new_place(&mut game);

        assert_eq!(&1, place.resistance.get(&AttackType::Physical).unwrap());
        assert_eq!(&3, place.reward.get(&TreasureType::Gold).unwrap());
        assert_eq!(place.resistance.len(), 1);
        assert_eq!(game.difficulty, place.item_reward_possible_rolls);
    }

    #[test]
    fn seeding_test() {
        let original_game = new_game_testing(Some([1; 16]));

        for _i in 1..1000 {
            let game = new_game_testing(Some([1; 16]));
            assert_eq!(original_game, game);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = new_game_testing(Some([1; 16]));

        for _i in 1..438 {
            new_place(&mut game);
        }
    }
}
