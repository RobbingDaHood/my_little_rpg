#[cfg(test)]
mod tests_int {
    use crate::command::r#move::ExecuteMoveCommandReport;
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::damage_types::DamageType;
    use crate::the_world::game::Game;
    use crate::the_world::modifier_gain::Gain;
    use crate::the_world::place::Place;
    use crate::{
        command::r#move::execute,
        generator::game::new_testing,
        the_world::{
            item_resource::Type,
            treasure_types::TreasureType::Gold,
        },
    };

    struct MoveCommandErrorBody {
        error_message: Box<str>,
        item_report: Box<str>,
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = new_testing(Some([1; 16]));
        let result = unwrap_move_error(execute(&mut game, 11));

        assert_eq!(
            Box::from(
                "Error: execute_move_command: Index 11 is out of range of places, places is 10 \
                 long."
            ),
            result.error_message
        );
        assert_eq!(Box::from("[]"), result.item_report);
        assert_eq!(None, game.treasure.get(&Gold));
    }

    #[test]
    fn test_execute_move_command_accumulate_resource_even_when_losing() {
        let (mut game, place) = standard_world_test_setup();
        move_and_verify_loss(&mut game);
        //Now there is enough mana to trigger the powerfull item
        let result = execute(&mut game, 0).expect("Test failed!");

        assert_eq!(
            "You won and got a new item in the inventory.",
            &*result.result
        );
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&Type::Mana));
        assert_eq!(10, game.inventory.len());
        assert_eq!(2, game.statistics.moves_count);
        assert_eq!(1, game.statistics.wins);
        assert_eq!(1, game.statistics.loses);
        assert_eq!(1, game.statistics.wins_in_a_row);
        assert_eq!(0, game.statistics.loses_in_a_row);
    }


    // TODO The above is how to fix all the tests; use that method.
    // TODO There is still an issue on how to assert the item reports, because at the moment they will all get printed; while I am only interested in the error.

    #[test]
    fn test_manually_adding_five_mana_gets_standard_setup_to_work() {
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);

        let result = execute(&mut game, 0).expect("Test failed!");

        assert_eq!(
            "You won and got a new item in the inventory.",
            &*result.result
        );
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&Type::Mana));
        assert_eq!(10, game.inventory.len());
        assert_eq!(1, game.statistics.moves_count);
        assert_eq!(1, game.statistics.wins);
        assert_eq!(0, game.statistics.loses);
        assert_eq!(1, game.statistics.wins_in_a_row);
        assert_eq!(0, game.statistics.loses_in_a_row);
    }

    #[test]
    fn test_add_flatdamage_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamage(DamageType::Physical, 2000));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_flatdamage_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamage(DamageType::Physical, 1000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_flatdamage_wrong_type_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamage(DamageType::Corruption, 2000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_increase_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamage(DamageType::Physical, 1900));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_percentage_increase_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamage(DamageType::Physical, 1899));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_increase_wrong_type_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamage(DamageType::Corruption, 1900));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_increase_too_early_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.insert(0, Gain::PercentageIncreaseDamage(DamageType::Physical, 1900));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_flatresistancereduction_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Physical, 2000));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_flatresistancereduction_wrong_type_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Corruption, 2000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_flatresistancereduction_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Physical, 1000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentageresistancereduction_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Physical, 1000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseResistanceReduction(DamageType::Physical, 90));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_percentageresistancereduction_wrong_type_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Physical, 1000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseResistanceReduction(DamageType::Corruption, 90));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentageresistancereduction_to_early_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseResistanceReduction(DamageType::Physical, 90));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Physical, 1000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentageresistancereduction_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatResistanceReduction(DamageType::Physical, 1000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseResistanceReduction(DamageType::Physical, 89));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_flatdamage_againstmaxresistance_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(2000));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_flatdamage_againstmaxresistance_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(1000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_againstmaxresistance_increase_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamageAgainstHighestResistance(1900));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_percentage_againstmaxresistance_increase_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamageAgainstHighestResistance(1899));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_againstmaxresistance_increase_too_early_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[0].modifiers[0].gains.push(Gain::PercentageIncreaseDamageAgainstHighestResistance(1899));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_flatdamage_againstminresistance_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.clear();
        game.places[0].resistance.insert(DamageType::Physical, 1999);
        game.places[0].resistance.insert(DamageType::Corruption, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstLowestResistance(1999));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_flatdamage_againstminresistance_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.clear();
        game.places[0].resistance.insert(DamageType::Physical, 1999);
        game.places[0].resistance.insert(DamageType::Corruption, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstLowestResistance(1000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_againstminresistance_increase_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.clear();
        game.places[0].resistance.insert(DamageType::Physical, 1999);
        game.places[0].resistance.insert(DamageType::Corruption, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstLowestResistance(1000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamageAgainstLowestResistance(82));

        move_and_verify_win(&mut game, place);
    }

    #[test]
    fn test_add_percentage_againstminresistance_increase_too_little_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.clear();
        game.places[0].resistance.insert(DamageType::Physical, 1999);
        game.places[0].resistance.insert(DamageType::Corruption, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstLowestResistance(1000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamageAgainstLowestResistance(81));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_againstminresistance_increase_too_early_does_not_work() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.clear();
        game.places[0].resistance.insert(DamageType::Physical, 1999);
        game.places[0].resistance.insert(DamageType::Corruption, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstHighestResistance(2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseDamageAgainstLowestResistance(82));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamageAgainstLowestResistance(1000));

        move_and_verify_loss(&mut game);
    }

    #[test]
    fn test_add_percentage_increase_treasure_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        assert_eq!(None, game.treasure.get(&Gold));
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamage(DamageType::Physical, 2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::PercentageIncreaseTreasure(Gold, 2000));

        execute(&mut game, 0).expect("Test failed!");
        assert_eq!(Some(&6468), game.treasure.get(&Gold));
    }

    #[test]
    fn test_add_items_reward_works() {
        // Based on test_manually_adding_five_mana_gets_standard_setup_to_work passing
        let (mut game, _place) = standard_world_test_setup();
        assert_eq!(None, game.treasure.get(&Gold));
        game.item_resources.insert(Type::Mana, 5);
        game.places[0].resistance.insert(DamageType::Physical, 2000);
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatDamage(DamageType::Physical, 2000));
        game.equipped_items[1].modifiers[0].gains.push(Gain::FlatIncreaseRewardedItems(2000));

        execute(&mut game, 0).expect("Test failed!");
        assert_eq!(2010, game.inventory.len());
    }

    // TODO add tests of gains and add seeding test after many moves

    fn unwrap_move_error(result: Result<ExecuteMoveCommandReport, MyError>) -> MoveCommandErrorBody {
        match result.unwrap_err() {
            MyError::MoveCommand { error_message, item_report } => MoveCommandErrorBody { error_message, item_report },
            _ => panic!("Did not return the right type of error!")
        }
    }

    fn move_and_verify_win(mut game: &mut Game, place: Place) {
        let result = execute(&mut game, 0).expect("Test failed!");

        assert_eq!(
            "You won and got a new item in the inventory.",
            &*result.result
        );
        assert_eq!(2, result.item_report.len());
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&Gold), game.treasure.get(&Gold));
        assert_ne!(&0, game.treasure.get(&Gold).unwrap());
        assert_eq!(Some(&1), game.item_resources.get(&Type::Mana));
        assert_eq!(10, game.inventory.len());
        assert_eq!(1, game.statistics.moves_count);
        assert_eq!(1, game.statistics.wins);
        assert_eq!(0, game.statistics.loses);
        assert_eq!(1, game.statistics.wins_in_a_row);
        assert_eq!(0, game.statistics.loses_in_a_row);
    }

    fn move_and_verify_loss(mut game: &mut Game) {
        let result = execute(&mut game, 0).expect_err("Test failed!");

        let printed_result = format!("{result:?}");
        println!("{printed_result}");

        assert!(printed_result.contains("You did not deal enough damage to overcome the challenges in this place."));
        assert_eq!(2, printed_result.matches("\\\"item\\\"").count());
        assert!(!printed_result.contains("Gold"));

        assert_eq!(9, game.inventory.len());
        assert_eq!(1, game.statistics.moves_count);
        assert_eq!(0, game.statistics.wins);
        assert_eq!(1, game.statistics.loses);
        assert_eq!(0, game.statistics.wins_in_a_row);
        assert_eq!(1, game.statistics.loses_in_a_row);
    }

    fn standard_world_test_setup() -> (Game, Place) {
        let game = new_testing(Some([1; 16]));
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&Gold));
        assert_eq!(None, game.item_resources.get(&Type::Mana));
        assert_eq!(9, game.inventory.len());
        assert_eq!(0, game.statistics.moves_count);
        assert_eq!(0, game.statistics.wins);
        assert_eq!(0, game.statistics.loses);
        assert_eq!(0, game.statistics.wins_in_a_row);
        assert_eq!(0, game.statistics.loses_in_a_row);
        (game, place)
    }
}
