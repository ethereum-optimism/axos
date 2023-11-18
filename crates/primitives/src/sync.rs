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
    #[cfg(feature = "alloc")]
    type Err = alloc::string::String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use alloc::string::ToString;
        match s {
            "fast" => Ok(Self::Fast),
            "checkpoint" => Ok(Self::Checkpoint),
            "challenge" => Ok(Self::Challenge),
            "full" => Ok(Self::Full),
            _ => Err("invalid sync mode".to_string()),
        }
    }
}
