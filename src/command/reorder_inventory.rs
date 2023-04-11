mod tests;

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
