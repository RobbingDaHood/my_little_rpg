#[cfg(test)]
mod tests_int {
    use std::fs;

    use crate::{
        command::{
            expand_max_element::execute,
            save_load::{execute_load_command, execute_save_command},
        },
        generator::game::new_testing,
        the_world::treasure_types::TreasureType::Gold,
    };

    #[test]
    fn save_the_world_special_chars() {
        // I implemented this test in an effort to trigger the saving errors, but could not. So I just converted it to a ordianry test instead.
        let game = new_testing(Some([1; 16]));
        assert_eq!(
            Box::from("You saved the world!"),
            execute_save_command(&game, "\"||||!!!save_load_seeding_test", Some("./testing/".into())).unwrap()
        );

        fs::remove_dir_all("./testing/").expect("Had trouble cleanup after save_load_time");
    }

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute(&mut game);

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            execute_save_command(&game, "save_load_seeding_test", Some("./testing2/".into())).unwrap();
            let mut parsed_game = execute_load_command("save_load_seeding_test", Some("./testing2/".into())).unwrap();

            assert_eq!(game, parsed_game);

            let result = execute(&mut parsed_game);
            assert_eq!(original_result, result);
        }

        //Cleanup
        fs::remove_dir_all("./testing2/").expect("Had trouble cleanup after save_load_time");
    }
}
