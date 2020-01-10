use crate::action::Action;
use crate::amount_limit::AmountLimit;
use crate::signature::Signature;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tx {
    /// Time of transaction. Unixepoch start in nanoseconds
    pub time: i64,
    /// Transaction expiration time. Unixepoch starts in nanoseconds. If the chunk node does not receive the transaction until after the expiration time, it will not execute
    pub expiration: i64,
    /// GAS multiplying rate. This transaction shall be paid according to the gas ratio of the default gas. The higher the multiplier, the higher the priority. The reasonable value range is [1.0, 100.0]
    pub gas_ratio: f64,
    /// The maximum allowed gas of the transaction, with a minimum setting of 50000
    pub gas_limit: f64,
    /// Used in delayed transactions. The number of nanoseconds to delay execution. Non delayed transaction set to 0
    pub delay: i64,
    /// Network ID
    pub chain_id: u32,
    /// Specific call in transaction
    pub actions: Vec<Action>,
    /// Token restrictions on transactions. You can specify multiple tokens and a corresponding number limit. If the transaction exceeds these limits, execution fails
    pub amount_limit: Vec<AmountLimit>,
    /// ID of the transaction sender
    pub publisher: String,
    /// Publisher's signature. The signing process is as follows. Publisher can provide multiple signatures with different permissions. You can refer to the documentation of the permission system
    pub publisher_sigs: Vec<Signature>,
    /// Signer ID other than publisher. It can be empty.
    pub signers: Vec<String>,
    /// Signature of signers. Each signer can have one or more signatures, so the length is not less than the length of signers
    pub signatures: Vec<Signature>
}
