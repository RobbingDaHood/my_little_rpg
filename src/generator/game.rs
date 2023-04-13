use std::collections::HashMap;

use rand::RngCore;
use rand::SeedableRng;
use rand_pcg::{Lcg64Xsh32, Pcg32};

use crate::generator::place::new as new_place;
use crate::parser::hex_encoder::encode_hex;
use crate::the_world::attack_types::AttackType;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::game_statistics::GameStatistics;
use crate::the_world::item::{CraftingInfo, Item};
use crate::the_world::item_modifier::Modifier;
use crate::the_world::item_resource::Type;
use crate::the_world::modifier_cost::Cost;
use crate::the_world::modifier_gain::Gain;
use crate::Game;

mod tests;

pub fn new(seed: Option<[u8; 16]>) -> Game {
    let mut min_resistance = HashMap::new();
    min_resistance.insert(AttackType::Physical, 1);

    let mut max_resistance = HashMap::new();
    max_resistance.insert(AttackType::Physical, 2);

    let difficulty = Difficulty {
        max_resistance,
        min_resistance,
        max_simultaneous_resistances: 1,
        min_simultaneous_resistances: 1,
    };

    //Simple item
    let equipped_items = vec![Item {
        modifiers: vec![Modifier {
            costs: Vec::new(),
            gains: vec![Gain::FlatDamage(AttackType::Physical, 2)],
        }],
        crafting_info: CraftingInfo {
            possible_rolls: difficulty.clone(),
            places_count: 1,
        },
    }];

    let (seed, random_generator) = create_random_generator(seed);

    let game_statistics = GameStatistics {
        moves_count: 0,
        wins: 0,
        loses: 0,
        wins_in_a_row: 0,
        loses_in_a_row: 0,
    };

    let mut game = Game {
        places: Vec::new(),
        equipped_items,
        difficulty,
        treasure: HashMap::new(),
        item_resources: HashMap::new(),
        inventory: Vec::new(),
        seed,
        random_generator_state: random_generator,
        game_statistics,
    };

    let new_place = new_place(&mut game);
    game.places.push(new_place);

    game
}

fn create_random_generator(seed: Option<[u8; 16]>) -> ([u8; 16], Lcg64Xsh32) {
    let seed = match seed {
        Some(seed) => seed,
        None => {
            let mut new_seed: [u8; 16] = [1; 16];
            Pcg32::from_entropy().fill_bytes(&mut new_seed);
            new_seed
        }
    };

    println!("Using seed: {}", encode_hex(&seed));

    let random_generator = Pcg32::from_seed(seed);
    (seed, random_generator)
}

#[allow(dead_code)]
pub fn new_testing(seed: Option<[u8; 16]>) -> Game {
    let mut min_resistance = HashMap::new();
    min_resistance.insert(AttackType::Fire, 2);
    min_resistance.insert(AttackType::Frost, 3);
    min_resistance.insert(AttackType::Lightning, 4);
    min_resistance.insert(AttackType::Light, 5);
    min_resistance.insert(AttackType::Darkness, 6);
    min_resistance.insert(AttackType::Nature, 7);
    min_resistance.insert(AttackType::Corruption, 8);
    min_resistance.insert(AttackType::Holy, 9);

    let mut max_resistance = HashMap::new();
    max_resistance.insert(AttackType::Fire, 20);
    max_resistance.insert(AttackType::Frost, 30);
    max_resistance.insert(AttackType::Lightning, 40);
    max_resistance.insert(AttackType::Light, 50);
    max_resistance.insert(AttackType::Darkness, 60);
    max_resistance.insert(AttackType::Nature, 70);
    max_resistance.insert(AttackType::Corruption, 80);
    max_resistance.insert(AttackType::Holy, 90);

    let difficulty = Difficulty {
        max_resistance,
        min_resistance,
        max_simultaneous_resistances: 22,
        min_simultaneous_resistances: 23,
    };

    let mut equipped_items = Vec::new();

    //Generator item
    let mut modifiers = Vec::new();
    let modifier = Modifier {
        costs: Vec::new(),
        gains: vec![Gain::FlatItemResource(Type::Mana, 5)],
    };
    modifiers.push(modifier);
    let item = Item {
        modifiers,
        crafting_info: CraftingInfo {
            possible_rolls: difficulty.clone(),
            places_count: 10,
        },
    };
    equipped_items.push(item);

    //Powerful item
    let mut modifiers = Vec::new();
    for attack_type in AttackType::get_all() {
        let modifier = Modifier {
            costs: vec![Cost::FlatItemResource(Type::Mana, 1)],
            gains: vec![Gain::FlatDamage(attack_type, 100)],
        };
        modifiers.push(modifier);
    }
    let item = Item {
        modifiers,
        crafting_info: CraftingInfo {
            possible_rolls: difficulty.clone(),
            places_count: 10,
        },
    };
    equipped_items.push(item);

    //fill inventory with basic items
    let mut inventory = Vec::new();
    for attack_type in AttackType::get_all() {
        inventory.push(Some(Item {
            modifiers: vec![Modifier {
                costs: Vec::new(),
                gains: vec![Gain::FlatDamage(attack_type, 1)],
            }],
            crafting_info: CraftingInfo {
                possible_rolls: difficulty.clone(),
                places_count: 10,
            },
        }));
    }

    let (seed, random_generator) = create_random_generator(seed);

    let game_statistics = GameStatistics {
        moves_count: 0,
        wins: 0,
        loses: 0,
        wins_in_a_row: 0,
        loses_in_a_row: 0,
    };

    let mut game = Game {
        places: Vec::new(),
        equipped_items,
        difficulty,
        treasure: HashMap::new(),
        item_resources: HashMap::new(),
        inventory,
        seed,
        random_generator_state: random_generator,
        game_statistics,
    };

    for _i in 0..10 {
        let new_place = new_place(&mut game);
        game.places.push(new_place);
    }

    game
}
