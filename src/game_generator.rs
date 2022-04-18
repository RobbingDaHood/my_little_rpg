use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::Item;
use crate::item_modifier::ItemModifier;
use crate::modifier_gain::ModifierGain;
use crate::place_generator::{generate_place, PlaceGeneratorInput};

pub fn generate_new_game() -> Game {
    let mut min_resistance = HashMap::new();
    min_resistance.insert(AttackType::Fire, 2);
    min_resistance.insert(AttackType::Frost, 3);
    min_resistance.insert(AttackType::Lightning, 4);
    min_resistance.insert(AttackType::Light, 5);
    min_resistance.insert(AttackType::Darkness, 6);
    min_resistance.insert(AttackType::Nature, 7);
    min_resistance.insert(AttackType::Corruption, 8);

    let mut max_resistance = HashMap::new();
    max_resistance.insert(AttackType::Fire, 20);
    max_resistance.insert(AttackType::Frost, 30);
    max_resistance.insert(AttackType::Lightning, 40);
    max_resistance.insert(AttackType::Light, 50);
    max_resistance.insert(AttackType::Darkness, 60);
    max_resistance.insert(AttackType::Nature, 70);
    max_resistance.insert(AttackType::Corruption, 80);
    max_resistance.insert(AttackType::Holy, 90);

    let place_generator_input = PlaceGeneratorInput { max_resistance, min_resistance };

    let mut places = Vec::new();
    for _i in 0..10 {
        places.push(generate_place(&place_generator_input));
    }

    let mut equipped_items = Vec::new();
    let mut modifiers = Vec::new();

    for attack_type in AttackType::get_all_attack_types() {
        let modifier = ItemModifier {
            costs: Vec::new(),
            gains: vec![ModifierGain::FlatDamage(attack_type, 100)],
        };
        modifiers.push(modifier);
    }

    let item = Item { modifiers };
    equipped_items.push(item);

    Game { places, equipped_items, place_generator_input }
}