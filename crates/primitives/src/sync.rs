#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Sync Mode
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncMode {
    /// Fast sync mode
    Fast,
    /// Checkpoint sync mode
    Checkpoint,
    /// Challenge sync mode
    Challenge,
    /// Full sync mode runs the derivation pipeline
    /// from the genesis block to tip.
    Full,
}

#[cfg(feature = "alloc")]
impl alloc::str::FromStr for SyncMode {
    type Err = crate::str::GenericString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fast" => Ok(Self::Fast),
            "checkpoint" => Ok(Self::Checkpoint),
            "challenge" => Ok(Self::Challenge),
            "full" => Ok(Self::Full),
            _ => Err(Self::Err::from("invalid sync mode")),
        }
    }
}
