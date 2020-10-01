use alloc::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NetWork {
    /// Node ID in the p2p network
    pub id: String,
    /// Peer count of the node
    pub peer_count: i32,
}
