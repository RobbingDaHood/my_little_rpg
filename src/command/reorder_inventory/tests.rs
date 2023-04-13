#[cfg(test)]
mod tests_int {
    use crate::{
        command::reorder_inventory::execute,
        generator::game::new_testing,
        the_world::{
            item::{CraftingInfo, Item},
            item_modifier::Modifier,
        },
    };

    #[test]
    fn test_execute_equip_item() {
        let mut game = new_testing(Some([1; 16]));
        let item = Some(Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: Vec::new(),
            }],
            crafting_info: CraftingInfo {
                possible_rolls: game.difficulty.clone(),
                places_count: game.places.len(),
            },
        });

        game.inventory = vec![item.clone(), None, item.clone(), None, None, item];

        assert_eq!(
            Into::<Box<str>>::into("Inventory is reordered."),
            execute(&mut game)
        );

        assert_eq!(3, game.inventory.len());
        assert_eq!(0, game.inventory.iter().filter(|i| i.is_none()).count());
    }
}
