use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameStatistics {
    pub(crate) moves_count: u64,
    pub(crate) wins: u64,
    pub(crate) loses: u64,
    pub(crate) wins_in_a_row: u64,
    pub(crate) loses_in_a_row: u64,
}