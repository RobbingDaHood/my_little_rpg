use std::collections::HashMap;
use crate::Game;
use crate::modifier_gain::ModifierGain;

pub fn execute_move_command(game: &mut Game, index: usize) -> String {
    if game.places().len() < index {
        return format!("Error: execute_move_command: Index {} is out of range of places, places is {} long.", index, game.places().len());
    }

    let mut current_damage = HashMap::new();

    for item in game.equipped_items() {
        for modifier in item.modifiers() {
            // if costs are paid

            for gain in modifier.gains() {
                match gain {
                    ModifierGain::FlatDamage(attack_type, amount) => {
                        *current_damage.entry(attack_type).or_insert(0) += amount;
                    }
                }
            }
        }
    }

    println!("{:?}", current_damage);

    "".to_string()
}

#[cfg(test)]
mod tests_int {
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_new_game;

    #[test]
    fn test_execute_move_command() {
        let mut game = generate_new_game();

        assert_eq!("", execute_move_command(&mut game, 0));
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = generate_new_game();

        assert_eq!("Error: execute_move_command: Index 11 is out of range of places, places is 10 long.", execute_move_command(&mut game, 11));
    }
}