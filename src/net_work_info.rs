use serde::{Serialize, Deserialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetWork {
    /// Node ID in the p2p network
    pub id: String,
    /// Peer count of the node
    pub peer_count: i32,

}



