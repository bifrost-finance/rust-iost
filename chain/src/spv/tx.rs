use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::{Action, AmountLimit};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Sign {
    pub algorithm: u8,
    pub sig: String,

    // #[cfg(feature = "std")]
    // #[serde(rename = "pubKey")]
    pub pub_key: String,
}

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Tx {
    /// Time of transaction. Unixepoch start in nanoseconds
    pub time: String,
    /// Transaction expiration time. Unixepoch starts in nanoseconds. If the chunk node does not receive the transaction until after the expiration time, it will not execute
    pub expiration: String,
    /// GAS multiplying rate. This transaction shall be paid according to the gas ratio of the default gas. The higher the multiplier, the higher the priority. The reasonable value range is [1.0, 100.0]
    #[cfg(feature = "std")]
    #[serde(rename = "gasRatio")]
    pub gas_ratio: String,
    /// The maximum allowed gas of the transaction, with a minimum setting of 50000
    #[cfg(feature = "std")]
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    /// Used in delayed transactions. The number of nanoseconds to delay execution. Non delayed transaction set to 0
    pub delay: String,
    /// Network ID
    pub chain_id: u32,
    /// Specific call in transaction
    pub actions: Vec<Action>,
    /// Token restrictions on transactions. You can specify multiple tokens and a corresponding number limit. If the transaction exceeds these limits, execution fails
    #[cfg(feature = "std")]
    #[serde(rename = "amountLimit")]
    pub amount_limit: Vec<AmountLimit>,
    /// ID of the transaction sender
    pub publisher: String,
    /// Publisher's signature. The signing process is as follows. Publisher can provide multiple signatures with different permissions. You can refer to the documentation of the permission system
    #[cfg(feature = "std")]
    #[serde(rename = "publishSigns")]
    pub publisher_sigs: Vec<Sign>,
    /// Signer ID other than publisher. It can be empty.
    pub signers: Vec<String>,
    /// Signature of signers. Each signer can have one or more signatures, so the length is not less than the length of signers
    // pub signatures: Vec<Signature>,
    #[cfg(feature = "std")]
    #[serde(rename = "referredTx")]
    pub referred_tx: Option<String>,
    pub reserved: Option<String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TxReceiptStatus {
    pub code: i32,
    pub message: String,
}
