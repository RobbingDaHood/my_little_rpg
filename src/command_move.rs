use std::collections::HashMap;
use crate::Game;
use crate::modifier_gain::ModifierGain;
use crate::place_generator::generate_place;

pub fn execute_move_command(game: &mut Game, index: usize) -> String {
    if game.places.len() < index {
        return format!("Error: execute_move_command: Index {} is out of range of places, places is {} long.", index, game.places.len());
    }

    let mut current_damage = HashMap::new();

    for item in &game.equipped_items {
        for modifier in &item.modifiers {
            // TODO: if costs are paid

            // TODO: return step by step what the items do.

            for gain in &modifier.gains {
                match gain {
                    ModifierGain::FlatDamage(attack_type, amount) => {
                        *current_damage.entry(attack_type.clone()).or_insert(0) += amount;
                    }
                }
            }
        }

        if let Some(rewards) = game.places.get(index)
            .expect("Error: execute_move_command: Could not find place even though it were within the index.")
            .claim_rewards(&current_damage) {

            for (treasure_type, amount) in rewards {
                *game.treasure.entry(treasure_type).or_insert(0) += amount;
            }

            game.places[index] = generate_place(&game.place_generator_input);
            return "We are winning".to_string();
        }
    }


    println!("{:?}", current_damage);

    "You did not deal enough damage to overcome the challenges in this place.".to_string()
}

#[cfg(test)]
mod tests_int {
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_new_game;
    use crate::treasure_types::TreasureType;

    #[test]
    fn test_execute_move_command() {
        let mut game = generate_new_game();
        let place = game.places[0].clone();
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));

        assert_eq!("We are winning", execute_move_command(&mut game, 0));
        assert_ne!(place, game.places[0]);
        assert_eq!(place.reward.get(&TreasureType::Gold), game.treasure.get(&TreasureType::Gold));
        assert_ne!(&0, game.treasure.get(&TreasureType::Gold).unwrap());
    }

    #[test]
    fn test_execute_move_command_index_out_of_bounds() {
        let mut game = generate_new_game();

        assert_eq!("Error: execute_move_command: Index 11 is out of range of places, places is 10 long.", execute_move_command(&mut game, 11));
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }

    #[test]
    fn test_execute_not_enough_damage() {
        let mut game = generate_new_game();
        game.equipped_items = Vec::new();

        assert_eq!("You did not deal enough damage to overcome the challenges in this place.", execute_move_command(&mut game, 0));
        assert_eq!(None, game.treasure.get(&TreasureType::Gold));
    }
}