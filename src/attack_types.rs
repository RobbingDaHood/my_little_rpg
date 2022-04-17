use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType::{Corruption, Darkness, Fire, Frost, Holy, Light, Lightning, Nature, Physical};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum AttackType {
    Physical,
    Fire,
    Frost,
    Lightning,
    Light,
    Darkness,
    Nature,
    Corruption,
    Holy,
}

impl AttackType {
    pub fn get_all_attack_types() -> Vec<AttackType> {
        vec![
            Physical,
            Fire,
            Frost,
            Lightning,
            Light,
            Darkness,
            Nature,
            Corruption,
            Holy,
        ]
    }
}