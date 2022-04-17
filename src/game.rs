use crate::place::Place;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    places: Vec<Place>,
}

impl Game {
    pub fn new(places: Vec<Place>) -> Self {
        Self { places }
    }
}