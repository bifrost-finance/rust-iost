use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VoteInfo {
    /// candidate
    pub option: String,
    /// number of votes
    pub votes: String,
    /// number of votes cleared
    pub cleared_votes: String,
}
