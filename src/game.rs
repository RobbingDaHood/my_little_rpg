use crate::place::Place;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    places: Vec<Place>,
}