use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteInfo {
    /// candidate
    pub option: String,
    /// number of votes
    pub votes: String,
    /// number of votes cleared
    pub cleared_votes: String
}
