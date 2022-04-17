
use serde::{Deserialize, Serialize};

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
    Holy
}