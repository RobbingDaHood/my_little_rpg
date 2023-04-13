#[cfg(test)]
mod tests_int {
    use crate::{
        command::craft_reroll_modifier::{execute, execute_craft_reroll_modifier_calculate_cost},
        generator::game::new_testing,
        my_little_rpg_errors::MyError,
        the_world::{index_specifier, item::test_util::create_item},
        Game,
    };

    #[test]
    fn test_execute_craft_item() {
        let mut game = new_testing(Some([1; 16]));

        insert_game_in_inventory(&mut game);

        let old_item = game.inventory[0].clone();
        assert_eq!(10, game.inventory.len());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "inventory_index 0 and index_specifier Absolute(0) cannot be the same".to_string()
            )),
            execute(
                &mut game,
                0,
                0,
                vec![
                    index_specifier::IndexSpecifier::Absolute(0),
                    index_specifier::IndexSpecifier::Absolute(1)
                ]
            )
        );
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "sacrifice_item_index 1 need to have at least 2 modifiers but it only had 1"
                    .to_string()
            )),
            execute(
                &mut game,
                0,
                1,
                vec![
                    index_specifier::IndexSpecifier::Absolute(1),
                    index_specifier::IndexSpecifier::Absolute(2)
                ]
            )
        );
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        assert_eq!(2, execute_craft_reroll_modifier_calculate_cost(&game, 0));
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "craft_reroll_modifier needs 2 items to be sacrificed but you only provided 1"
                    .to_string()
            )),
            execute(
                &mut game,
                0,
                0,
                vec![index_specifier::IndexSpecifier::Absolute(1)]
            )
        );
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(10, game.inventory.len());

        let result = execute(
            &mut game,
            0,
            0,
            vec![
                index_specifier::IndexSpecifier::Absolute(1),
                index_specifier::IndexSpecifier::Absolute(2),
            ],
        );
        assert!(result.is_ok());
        assert_ne!(old_item.unwrap(), result.unwrap().new_item);
        let old_item = game.inventory[0].clone();
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "inventory_index 99 is not within the range of the inventory 10".to_string()
            )),
            execute(
                &mut game,
                99,
                0,
                vec![index_specifier::IndexSpecifier::Absolute(0)]
            )
        );
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "modifier_index 99 is not within the range of the item modifiers 2".to_string()
            )),
            execute(
                &mut game,
                0,
                99,
                vec![index_specifier::IndexSpecifier::Absolute(0)]
            )
        );
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier Absolute(99) is not within the range of the inventory 10"
                    .to_string()
            )),
            execute(
                &mut game,
                0,
                0,
                vec![
                    index_specifier::IndexSpecifier::Absolute(99),
                    index_specifier::IndexSpecifier::Absolute(1)
                ]
            )
        );
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "inventory_index 1 is empty.".to_string()
            )),
            execute(
                &mut game,
                1,
                0,
                vec![
                    index_specifier::IndexSpecifier::Absolute(4),
                    index_specifier::IndexSpecifier::Absolute(5)
                ]
            )
        );
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier Absolute(1) is pointing at empty inventory slot.".to_string()
            )),
            execute(
                &mut game,
                0,
                0,
                vec![
                    index_specifier::IndexSpecifier::Absolute(1),
                    index_specifier::IndexSpecifier::Absolute(2)
                ]
            )
        );
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier Absolute(8) is already present in calculated sacrifice indexes \
                 {8}"
                .to_string()
            )),
            execute(
                &mut game,
                0,
                0,
                vec![
                    index_specifier::IndexSpecifier::Absolute(8),
                    index_specifier::IndexSpecifier::Absolute(8)
                ]
            )
        );
        assert_eq!(old_item, game.inventory[0]);
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());
    }

    #[test]
    fn test_execute_craft_item_positive() {
        let mut game = new_testing(Some([1; 16]));

        insert_game_in_inventory(&mut game);

        let result = execute(
            &mut game,
            0,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativePositive(1),
                index_specifier::IndexSpecifier::RelativePositive(2),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            0,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativePositive(1),
                index_specifier::IndexSpecifier::RelativePositive(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(6, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            0,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativePositive(1),
                index_specifier::IndexSpecifier::RelativePositive(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(4, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            0,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativePositive(1),
                index_specifier::IndexSpecifier::RelativePositive(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier: RelativePositive(1) did not find any items in inventory from \
                 relative point 1 until end of inventory."
                    .to_string()
            )),
            execute(
                &mut game,
                0,
                0,
                vec![
                    index_specifier::IndexSpecifier::RelativePositive(1),
                    index_specifier::IndexSpecifier::RelativePositive(1)
                ]
            )
        );
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());
    }

    fn insert_game_in_inventory(game: &mut Game) {
        game.inventory.insert(0, Some(create_item(game)));
    }

    #[test]
    fn test_execute_craft_item_negative() {
        let mut game = new_testing(Some([1; 16]));

        game.inventory.push(Some(create_item(&game)));

        let result = execute(
            &mut game,
            9,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativeNegative(1),
                index_specifier::IndexSpecifier::RelativeNegative(2),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            9,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativeNegative(1),
                index_specifier::IndexSpecifier::RelativeNegative(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(6, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            9,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativeNegative(1),
                index_specifier::IndexSpecifier::RelativeNegative(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(4, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            9,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativeNegative(1),
                index_specifier::IndexSpecifier::RelativeNegative(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier: RelativeNegative(1) did not find any items in inventory from \
                 relative point 8 until start of inventory."
                    .to_string()
            )),
            execute(
                &mut game,
                9,
                0,
                vec![
                    index_specifier::IndexSpecifier::RelativeNegative(1),
                    index_specifier::IndexSpecifier::RelativeNegative(1)
                ]
            )
        );
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());
    }

    #[test]
    fn test_execute_craft_item_mixed() {
        let mut game = new_testing(Some([1; 16]));

        game.inventory.insert(5, Some(create_item(&game)));

        let result = execute(
            &mut game,
            5,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativeNegative(1),
                index_specifier::IndexSpecifier::RelativePositive(2),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(8, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            5,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativePositive(1),
                index_specifier::IndexSpecifier::RelativeNegative(1),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(6, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            5,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativePositive(1),
                index_specifier::IndexSpecifier::Absolute(0),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(4, game.inventory.iter().filter(|i| i.is_some()).count());

        let result = execute(
            &mut game,
            5,
            0,
            vec![
                index_specifier::IndexSpecifier::RelativeNegative(1),
                index_specifier::IndexSpecifier::Absolute(9),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier: RelativePositive(1) did not find any items in inventory from \
                 relative point 6 until end of inventory."
                    .to_string()
            )),
            execute(
                &mut game,
                5,
                0,
                vec![
                    index_specifier::IndexSpecifier::RelativePositive(1),
                    index_specifier::IndexSpecifier::RelativeNegative(1)
                ]
            )
        );
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier: RelativePositive(1) did not find any items in inventory from \
                 relative point 6 until end of inventory."
                    .to_string()
            )),
            execute(
                &mut game,
                5,
                0,
                vec![
                    index_specifier::IndexSpecifier::RelativeNegative(1),
                    index_specifier::IndexSpecifier::RelativePositive(1)
                ]
            )
        );
        assert_eq!(2, game.inventory.iter().filter(|i| i.is_some()).count());

        game.inventory[2] = None;
        assert_eq!(
            Err(MyError::create_execute_command_error(
                "index_specifier: RelativeNegative(1) did not find any items in inventory from \
                 relative point 4 until start of inventory."
                    .to_string()
            )),
            execute(
                &mut game,
                5,
                0,
                vec![
                    index_specifier::IndexSpecifier::RelativeNegative(1),
                    index_specifier::IndexSpecifier::RelativeNegative(1)
                ]
            )
        );
    }

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        let original_result = execute(
            &mut game,
            0,
            0,
            vec![index_specifier::IndexSpecifier::Absolute(1)],
        );

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            let result = execute(
                &mut game,
                0,
                0,
                vec![index_specifier::IndexSpecifier::Absolute(1)],
            );
            assert_eq!(original_result, result);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = new_testing(Some([1; 16]));

        for i in 1..438 {
            game.inventory.push(Some(create_item(&game)));
            assert!(execute(
                &mut game,
                0,
                0,
                vec![index_specifier::IndexSpecifier::Absolute(i)]
            )
            .is_ok());
        }
    }
}
