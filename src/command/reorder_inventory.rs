use serde_json::{json, Value};

use crate::Game;

mod tests;

pub fn execute_json(game: &mut Game) -> Value {
    json!(execute(game))
}

pub fn execute(game: &mut Game) -> Box<str> {
    game.inventory.retain(Option::is_some);
    "Inventory is reordered.".into()
}
