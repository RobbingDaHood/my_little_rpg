use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameStatistics {
    pub(crate) moves_count: u64,
}