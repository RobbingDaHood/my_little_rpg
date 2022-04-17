use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::Item;
use crate::item_modifier::ItemModifier;
use crate::modifier_gain::ModifierGain;
use crate::place_generator::{generate_place, PlaceGeneratorInput};

pub fn generate_new_game() -> Game {
    let mut min = HashMap::new();
    min.insert(AttackType::Fire, 2);
    min.insert(AttackType::Frost, 3);
    min.insert(AttackType::Lightning, 4);
    min.insert(AttackType::Light, 5);
    min.insert(AttackType::Darkness, 6);
    min.insert(AttackType::Nature, 7);
    min.insert(AttackType::Corruption, 8);

    let mut max = HashMap::new();
    max.insert(AttackType::Fire, 20);
    max.insert(AttackType::Frost, 30);
    max.insert(AttackType::Lightning, 40);
    max.insert(AttackType::Light, 50);
    max.insert(AttackType::Darkness, 60);
    max.insert(AttackType::Nature, 70);
    max.insert(AttackType::Corruption, 80);
    max.insert(AttackType::Holy, 90);

    let input = PlaceGeneratorInput::new(max, min);

    let mut places = Vec::new();
    for _i in 0..10 {
        places.push(generate_place(&input));
    }

    let mut equipped_items = Vec::new();
    let mut modifiers = Vec::new();

    for attack_type in AttackType::get_all_attack_types() {
        let modifier = ItemModifier::new(Vec::new(), vec![ModifierGain::FlatDamage(attack_type, 100)]);
        modifiers.push(modifier);
    }

    let item = Item::new(modifiers);
    equipped_items.push(item);

    Game::new(places, equipped_items)
}