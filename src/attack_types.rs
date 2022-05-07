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

//TODO: Half unsatisfied defence, double unsatisfied attack, double tressure, doulbe items (Physical does not need to do anything).

impl AttackType {
    pub fn get_all() -> Vec<AttackType> {
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