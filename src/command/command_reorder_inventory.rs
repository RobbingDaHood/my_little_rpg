use crate::Game;
use crate::the_world::item::Item;

pub fn execute_reorder_inventory(game: &mut Game) -> String {
    let reordered_inventory = game.inventory.clone().into_iter()
        .filter(Option::is_some)
        .collect::<Vec<Option<Item>>>();

    game.inventory = reordered_inventory;

    "Inventory is reordered.".to_string()
}


#[cfg(test)]
mod tests_int {
    use crate::command::command_reorder_inventory::execute_reorder_inventory;
    use crate::generator::game_generator::generate_testing_game;
    use crate::the_world::item::{CraftingInfo, Item};
    use crate::the_world::item_modifier::Modifier;

    #[test]
    fn test_execute_equip_item() {
        let mut game = generate_testing_game(Some([1; 16]));
        let item = Some(Item {
            modifiers: vec![
                Modifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        });

        game.inventory = vec![item.clone(), None, item.clone(), None, None, item];

        assert_eq!("Inventory is reordered.".to_string(), execute_reorder_inventory(&mut game));

        assert_eq!(3, game.inventory.len());
        assert_eq!(0, game.inventory.iter().filter(|i| i.is_none()).count());
    }
}