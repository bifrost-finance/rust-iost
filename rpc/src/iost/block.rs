pub struct Block {
    /// block hash
    pub head: Head,
    /// block version number
    pub sign: Sign,
    /// the hash of the parent block of this block
    pub parent_hash: String,
    /// the merkle tree hash of all transactions
    pub tx_merkle_hash: String,
    /// the merkle tree hash of all receipts
    pub tx_receipt_merkle_hash: String,
    /// block number
    pub number: String,
    /// public key of the block producer
    pub receipts: Vec<TxReceipt>,
    /// time of block production
    pub txs: Vec<>,

    /// transaction number in the block
    pub tx_hashes: Vec<String>,
    /// (This key is reserved.)
    // #[serde(deserialize_with = "info_or_null")]
    pub receipt_hashes: Vec<String>,

    pub block_type: String,
}

pub struct Sign {
    pub algorithm: u8,
    pub sig: String,
    pub pub_key: String,
}

pub struct TxReceipt {
    pub tx_hash: String,
    pub gas_usage: String,
    pub ram_usage: String,
    pub status: TxReceiptStatus,
    pub returns: Vec<String>,
    pub receipts: Vec<crate::receipt::Receipt>,
}

pub struct TxReceiptStatus {
    pub code: i32,
    pub message: String,
}