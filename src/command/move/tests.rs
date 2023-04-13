#[cfg(test)]
mod tests_int {
    use crate::{
        command::r#move::execute,
        generator::game::new_testing,
        the_world::{
            attack_types::AttackType,
            item::{CraftingInfo, Item},
            item_modifier::Modifier,
            item_resource::Type,
            modifier_cost::Cost,
            modifier_gain::Gain,
            treasure_types::TreasureType::Gold,
        },
        Game,
    };

    #[test]
    fn test_execute_move_command() {
        let mut game = new_testing(Some([1; 16]));
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(None, game.item_resources.get(&Type::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(0, game.game_statistics.moves_count);
        assert_eq!(0, game.game_statistics.wins);
        assert_eq!(0, game.game_statistics.loses);
        assert_eq!(0, game.game_statistics.wins_in_a_row);
        assert_eq!(0, game.game_statistics.loses_in_a_row);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(2, result.item_report.len());
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(Some(&5), game.item_resources.get(&Type::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(1, game.game_statistics.moves_count);
        assert_eq!(0, game.game_statistics.wins);
        assert_eq!(1, game.game_statistics.loses);
        assert_eq!(0, game.game_statistics.wins_in_a_row);
        assert_eq!(1, game.game_statistics.loses_in_a_row);

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&Type::Mana));
        assert_eq!(10, game.inventory.len());
        assert_eq!(2, game.game_statistics.moves_count);
        assert_eq!(1, game.game_statistics.wins);
        assert_eq!(1, game.game_statistics.loses);
        assert_eq!(1, game.game_statistics.wins_in_a_row);
        assert_eq!(0, game.game_statistics.loses_in_a_row);
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = new_testing(Some([1; 16]));

        let result = execute(&mut game, 11);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!(
            Into::<Box<str>>::into(
                "Error: execute_move_command: Index 11 is out of range of places, places is 10 \
                 long."
            ),
            result.result
        );
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&Gold));
    }

    #[test]
    fn test_execute_not_enough_damage() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();

        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(0, result.item_report.len());
        assert_eq!(None, game.treasure.get(&Gold));
    }

    #[test]
    fn test_execute_move_command_item_after_claim_does_not_activate() {
        let mut game = new_testing(Some([1; 16]));
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(None, game.item_resources.get(&Type::Mana));

        let power_item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: AttackType::get_all()
                    .iter()
                    .map(|attack_type| Gain::FlatDamage(attack_type.clone(), 100))
                    .collect(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };
        game.equipped_items.insert(0, power_item);

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(1, result.item_report.len()); //Only the first item got activated, because that were enough.
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(None, game.item_resources.get(&Type::Mana));

        //Putting the power item at the end
        game.equipped_items.swap(0, 2);
        game.equipped_items.swap(0, 1);

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(3, result.item_report.len()); //Now all three have a report.
        assert_ne!(place, game.places[0]);
        assert!(place.reward.get(&Gold).unwrap() < game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&5), game.item_resources.get(&Type::Mana));
    }

    #[test]
    fn test_flat_min_attack_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMinAttackRequirement(AttackType::Physical, 20)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamage(AttackType::Physical, 20)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMinAttackRequirement of 20 Physical damage, only did {} damage.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[2].effect_description
        );
    }

    #[test]
    fn test_flat_max_attack_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMaxAttackRequirement(AttackType::Physical, 1)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamage(AttackType::Physical, 3)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMaxAttackRequirement of 1 Physical damage, did {Physical: 3} damage and that \
                 is too much.\" } }"
            ),
            result.item_report[2].effect_description
        );
    }

    #[test]
    fn test_place_limited_by_index_modulus_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::PlaceLimitedByIndexModulus(6, vec![1, 3, 4])],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 PlaceLimitedByIndexModulus: 0 % 6 = 0 and that is not contained in [1, 3, 4].\" \
                 } }"
            ),
            execute(&mut game, 0).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            execute(&mut game, 1).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 PlaceLimitedByIndexModulus: 2 % 6 = 2 and that is not contained in [1, 3, 4].\" \
                 } }"
            ),
            execute(&mut game, 2).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            execute(&mut game, 3).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            execute(&mut game, 4).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 PlaceLimitedByIndexModulus: 5 % 6 = 5 and that is not contained in [1, 3, 4].\" \
                 } }"
            ),
            execute(&mut game, 5).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 PlaceLimitedByIndexModulus: 6 % 6 = 0 and that is not contained in [1, 3, 4].\" \
                 } }"
            ),
            execute(&mut game, 6).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            execute(&mut game, 7).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 PlaceLimitedByIndexModulus: 8 % 6 = 2 and that is not contained in [1, 3, 4].\" \
                 } }"
            ),
            execute(&mut game, 8).unwrap_err().item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            execute(&mut game, 9).unwrap_err().item_report[0].effect_description
        );
    }

    #[test]
    fn test_flat_item_resource() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatItemResource(Type::Mana, 20)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = item_with_gains(&mut game);

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Were not able to pay all the \
                 costs. Had to pay {Mana: 20}, but only had {} available.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[2].effect_description
        );
    }

    fn item_with_gains(game: &mut Game) -> Item {
        Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatItemResource(Type::Mana, 20)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        }
    }

    #[test]
    fn test_flat_min_sum_attack_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatSumMinAttackRequirement(20)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamage(AttackType::Physical, 10)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatSumMinAttackRequirement of 20 damage, only did {} damage.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatSumMinAttackRequirement of 20 damage, only did {Physical: 10} damage.\" } }"
            ),
            result.item_report[2].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[3].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[4].effect_description
        );
    }

    #[test]
    fn test_flat_max_sum_attack_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatSumMaxAttackRequirement(20)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamage(AttackType::Physical, 11)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[2].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[3].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatSumMaxAttackRequirement of 20 damage, did {Physical: 22} damage damage and \
                 that is too much.\" } }"
            ),
            result.item_report[4].effect_description
        );
    }

    //TODO add tests that check several execute_move_command in a row, for the item resource accumulation.

    #[test]
    fn test_flat_min_sum_item_resource_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMinItemResourceRequirement(Type::Mana, 20)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = item_with_gains(&mut game);

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMinItemResourceRequirement of 20 Mana, only had 0.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[2].effect_description
        );
    }

    #[test]
    fn test_flat_max_sum_item_resource_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMaxItemResourceRequirement(Type::Mana, 20)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        let second_item_generates_needed_resource = item_with_gains(&mut game);

        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource.clone());
        game.equipped_items.push(first_item_cannot_pay.clone());
        game.equipped_items
            .push(second_item_generates_needed_resource);
        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[2].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[3].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMaxItemResourceRequirement of 20 Mana, had 40 and that is too much.\" } }"
            ),
            result.item_report[4].effect_description
        );
    }

    #[test]
    fn test_flat_min_resistance_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMinResistanceRequirement(AttackType::Fire, 18)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMinResistanceRequirement of 18 Fire damage, place only has 17 damage.\" } }"
            ),
            result.item_report[0].effect_description
        );

        let result = execute(&mut game, 7);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
    }

    #[test]
    fn test_flat_max_resistance_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMaxResistanceRequirement(AttackType::Fire, 17)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );

        let result = execute(&mut game, 7);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMaxResistanceRequirement of 17 Fire damage, place has 20 damage and that is \
                 too much.\" } }"
            ),
            result.item_report[0].effect_description
        );
    }

    #[test]
    fn test_flat_min_sum_resistance_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMinSumResistanceRequirement(195)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMinSumResistanceRequirement of 195 damage, place only has 194 damage.\" } }"
            ),
            result.item_report[0].effect_description
        );

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
    }

    #[test]
    fn test_flat_max_sum_resistance_requirement() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::FlatMaxSumResistanceRequirement(194)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(first_item_cannot_pay);

        let result = execute(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 FlatMaxSumResistanceRequirement of 194 damage, place has 241 damage and that is \
                 too much.\" } }"
            ),
            result.item_report[0].effect_description
        );
    }

    #[test]
    fn test_min_win_row_requirement() {
        let mut game = new_testing(Some([1; 16]));

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::MinWinsInARow(1)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.insert(0, first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 MinWinsInARow of 1 win, only hase 0 wins in a row.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 MinWinsInARow of 1 win, only hase 0 wins in a row.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );

        let result = execute(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
    }

    #[test]
    fn test_max_win_row_requirement() {
        let mut game = new_testing(Some([1; 16]));

        let first_item_cannot_pay = Item {
            modifiers: vec![Modifier {
                costs: vec![Cost::MaxWinsInARow(0)],
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.insert(0, first_item_cannot_pay);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );

        let result = execute(&mut game, 9);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into(
                "MyError { kind: ExecuteCommand { error_message: \"Did not fulfill the \
                 MaxWinsInARow of 0 win, have 1 wins in a row and that is too much.\" } }"
            ),
            result.item_report[0].effect_description
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[1].effect_description
        );
    }

    #[test]
    fn test_percentage_increase_damage() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![
                    Gain::FlatDamage(AttackType::Physical, 200),
                    Gain::PercentageIncreaseDamage(AttackType::Physical, 200),
                ],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            600,
            *result.item_report[0]
                .current_damage
                .get(&AttackType::Physical)
                .unwrap()
        );
    }

    #[test]
    fn test_flat_resistance_reduction_damage() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatResistanceReduction(AttackType::Physical, 200)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            200,
            *result.item_report[0]
                .current_resistance_reduction
                .get(&AttackType::Physical)
                .unwrap()
        );
    }

    #[test]
    fn test_percentage_increase_resistance_reduction() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![
                    Gain::FlatResistanceReduction(AttackType::Physical, 200),
                    Gain::PercentageIncreaseResistanceReduction(AttackType::Physical, 200),
                ],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            600,
            *result.item_report[0]
                .current_resistance_reduction
                .get(&AttackType::Physical)
                .unwrap()
        );
    }

    #[test]
    fn test_flat_damage_against_highest_resistance() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamageAgainstHighestResistance(200)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            200,
            *result.item_report[0]
                .current_damage
                .get(&AttackType::Holy)
                .unwrap()
        );
    }

    #[test]
    fn test_percentage_increase_damage_against_highest_resistance() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![
                    Gain::FlatDamageAgainstHighestResistance(200),
                    Gain::PercentageIncreaseDamageAgainstHighestResistance(200),
                ],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            600,
            *result.item_report[0]
                .current_damage
                .get(&AttackType::Holy)
                .unwrap()
        );
    }

    #[test]
    fn test_flat_damage_against_lowest_resistance() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamageAgainstLowestResistance(200)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            200,
            *result.item_report[0]
                .current_damage
                .get(&AttackType::Darkness)
                .unwrap()
        );
    }

    #[test]
    fn test_flat_damage_against_lowest_resistance_multiple() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![
                    Gain::FlatDamageAgainstLowestResistance(200),
                    Gain::FlatDamageAgainstLowestResistance(200),
                ],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            400,
            *result.item_report[0]
                .current_damage
                .get(&AttackType::Darkness)
                .unwrap()
        );
    }

    #[test]
    fn test_percentage_increase_damage_against_lowest_resistance() {
        let mut game = new_testing(Some([1; 16]));
        game.equipped_items = Vec::new();

        let item = Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![
                    Gain::FlatDamageAgainstLowestResistance(200),
                    Gain::PercentageIncreaseDamageAgainstLowestResistance(200),
                ],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        };

        game.equipped_items.push(item);

        let result = execute(&mut game, 0);

        assert!(result.is_err());

        let result = result.unwrap_err();
        assert_eq!(
            Into::<Box<str>>::into(
                "You did not deal enough damage to overcome the challenges in this place."
            ),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            600,
            *result.item_report[0]
                .current_damage
                .get(&AttackType::Darkness)
                .unwrap()
        );
    }

    #[test]
    fn test_percentage_increase_treasure() {
        let mut game = new_testing(Some([1; 16]));

        //Remove all costs of super item
        for modifier in &mut game.equipped_items[1].modifiers {
            modifier.costs = Vec::new();
        }

        game.equipped_items[1].modifiers[0]
            .gains
            .push(Gain::PercentageIncreaseTreasure(Gold, 200));
        game.equipped_items[1].modifiers[0]
            .gains
            .push(Gain::PercentageIncreaseTreasure(Gold, 300));

        let old_place = game.places[0].clone();

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(
            500,
            *result.item_report[1].treasure_bonus.get(&Gold).unwrap()
        );

        assert_eq!(
            old_place.reward.get(&Gold).unwrap() * 6,
            *game.treasure.get(&Gold).unwrap()
        );
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
    }

    #[test]
    fn test_increase_item_gain() {
        let mut game = new_testing(Some([1; 16]));

        //Remove all costs of super item
        for modifier in &mut game.equipped_items[1].modifiers {
            modifier.costs = Vec::new();
        }

        game.equipped_items[1].modifiers[0]
            .gains
            .push(Gain::FlatIncreaseRewardedItems(200));

        let old_inventory_count = game.inventory.len();

        let result = execute(&mut game, 0);

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            Into::<Box<str>>::into("You won and got a new item in the inventory."),
            result.result
        );
        assert_eq!(
            Into::<Box<str>>::into("Costs paid and all gains executed."),
            result.item_report[0].effect_description
        );
        assert_eq!(201, result.item_report[1].item_gain);

        assert_eq!(old_inventory_count + 201, game.inventory.len());
    }
}
