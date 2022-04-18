use crate::attack_types::AttackType;
use crate::Game;
use crate::item::Item;
use crate::item_modifier::ItemModifier;
use crate::modifier_gain::ModifierGain;

pub fn execute_create_item(game: &mut Game) -> Item {
    let new_item = Item {
        modifiers: vec![
            ItemModifier {
                costs: Vec::new(),
                gains: vec![ModifierGain::FlatDamage(AttackType::Physical, 1)],
            }
        ]
    };
    game.inventory.push(new_item.clone());
    new_item
}