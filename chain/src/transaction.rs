use alloc::string::String;
use alloc::vec::Vec;

use crate::{
    Action, AmountLimit, NumberBytes, Read, ReadError, Signature, TxReceipt, Write, WriteError,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Transaction {
    /// transaction's hash
    pub hash: String,
    /// timestamp of the transaction
    pub time: i64,
    /// the expiration of the transaction
    pub expiration: i64,
    /// GAS ratio, we recommend it to be 1.00 (1.00 – 100.00). Raise the ratio to let the network pack it faster
    pub gas_ratio: f64,
    /// Upper limits of GAS. This transaction will never cost more GAS than this amount
    pub gas_limit: f64,
    /// Transactions will be delayed by this much, in nanosecond
    pub delay: i64,
    /// id of blockchain on which the transaction could be executed
    pub chain_id: i32,
    /// the smallest transaction execution unit
    pub actions: Vec<Action>,
    /// list of transaction signatures
    pub signers: Vec<String>,
    /// sender of the transaction, who is responsible for fees
    pub publisher: String,
    /// dependency of transaction generation; used for delayed transactions
    pub referred_tx: String,
    /// Users may specify token limits. For example, {"iost": 100} specifies each signers will not spend more than 100 IOST for the transaction
    pub amount_limit: Vec<AmountLimit>,
    /// The signatures of signers
    pub signatures: Vec<Signature>,
    // the receipt of the transaction Action
    // pub tx_receipt: TxReceipt,
}
