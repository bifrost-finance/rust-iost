use crate::info::Info;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    /// block hash
    pub hash: String,
    /// block version number
    pub version: String,
    /// the hash of the parent block of this block
    pub parent_hash: String,
    /// the merkle tree hash of all transactions
    pub tx_merkle_hash: String,
    /// the merkle tree hash of all receipts
    pub tx_receipt_merkle_hash: String,
    /// block number
    pub number: String,
    /// public key of the block producer
    pub witness: String,
    /// time of block production
    pub time: String,
    /// total GAS consumption within the block
    pub gas_usage: f64,
    /// transaction number in the block
    pub tx_count: String,
    /// (This key is reserved.)
    pub info: Info,
    /// all the transactions.
    pub transactions: Vec<Transaction>
}

