use std::collections::HashMap;
use crate::attack_types::AttackType;
use crate::Game;
use crate::item::Item;
use crate::item_modifier::ItemModifier;
use crate::modifier_gain::ModifierGain;
use serde::{Deserialize, Serialize};
use crate::treasure_types::TreasureType;
use crate::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteCreateItemReport {
    pub(crate) new_item: Item,
    cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_create_item(game: &mut Game) -> ExecuteCreateItemReport {
    let new_item = Item {
        modifiers: vec![
            ItemModifier {
                costs: Vec::new(),
                gains: vec![ModifierGain::FlatDamage(AttackType::Physical, 1)],
            }
        ]
    };
    game.inventory.push(new_item.clone());

    ExecuteCreateItemReport {
        new_item,
        cost: HashMap::from([(Gold, 0)]),
        leftover_spending_treasure: game.treasure.clone(),
    }
}