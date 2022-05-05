use crate::Game;
use crate::item::Item;

pub fn execute_reorder_inventory(game: &mut Game) -> Result<String, String> {
    let reordered_inventory = game.inventory.clone().into_iter()
        .filter(|i| i.is_some())
        .collect::<Vec<Option<Item>>>();

    game.inventory = reordered_inventory;

    Ok("Inventory is reordered.".to_string())
}


#[cfg(test)]
mod tests_int {
    use crate::command_reorder_inventory::execute_reorder_inventory;
    use crate::game_generator::generate_testing_game;
    use crate::item::{CraftingInfo, Item};
    use crate::item_modifier::ItemModifier;

    #[test]
    fn test_execute_equip_item() {
        let mut game = generate_testing_game(Some([1; 16]));
        let item = Some(Item {
            modifiers: vec![
                ItemModifier {
                    costs: Vec::new(),
                    gains: Vec::new(),
                }
            ],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone()
            },
        });

        game.inventory = vec![item.clone(), None, item.clone(), None, None, item.clone()];

        assert_eq!(Ok("Inventory is reordered.".to_string()), execute_reorder_inventory(&mut game));

        assert_eq!(3, game.inventory.len());
        assert_eq!(0, game.inventory.iter().filter(|i| i.is_none()).count());
    }
}