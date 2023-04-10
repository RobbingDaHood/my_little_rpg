use serde_json::{json, Value};

use crate::Game;
use crate::the_world::item::Item;

pub fn execute_json(game: &mut Game) -> Value {
    json!(execute(game))
}

pub fn execute(game: &mut Game) -> Box<str> {
    let reordered_inventory = game.inventory.clone().into_iter()
        .filter(Option::is_some)
        .collect::<Vec<Option<Item>>>();

    game.inventory = reordered_inventory;

    "Inventory is reordered.".into()
}


#[cfg(test)]
mod tests_int {
    use crate::command::reorder_inventory::execute;
    use crate::generator::game::new_testing;
    use crate::the_world::item::{CraftingInfo, Item};
    use crate::the_world::item_modifier::Modifier;

    #[test]
    fn test_execute_equip_item() {
        let mut game = new_testing(Some([1; 16]));
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

        assert_eq!(Into::<Box<str>>::into("Inventory is reordered."), execute(&mut game));

        assert_eq!(3, game.inventory.len());
        assert_eq!(0, game.inventory.iter().filter(|i| i.is_none()).count());
    }
}