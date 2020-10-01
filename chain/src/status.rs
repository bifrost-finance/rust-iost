#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Status {
    PENDING,
    PACKED,
    IRREVERSIBLE,
    APPROVED,
}
