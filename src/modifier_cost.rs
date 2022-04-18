use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ModifierCost {
    FlatItemResource(ItemResourceType, u64),
}