#![allow(dead_code)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::{spv::tx::TxReceiptStatus, Receipt};

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TxReceipt {
    #[cfg(feature = "std")]
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    #[cfg(feature = "std")]
    #[serde(rename = "gasUsage")]
    pub gas_usage: String,
    #[cfg(feature = "std")]
    #[serde(rename = "ramUsage")]
    pub ram_usage: BTreeMap<String, String>,
    pub status: TxReceiptStatus,
    pub returns: Vec<String>,
    pub receipts: Vec<Receipt>,
}
// pub struct TxReceipt {
//     /// hash of the transaction
//     pub tx_hash: String,
//     /// GAS consumption of the transaction
//     pub gas_usage: f64,
//     /// RAM consumption for the transaction. map-key is account name, and value is RAM amount
//     pub ram_usage: BTreeMap<String, i64>,
//     /// Status of the transaction. SUCCESS; GAS_RUN_OUT - insufficient GAS;
//     /// BALANCE_NOT_ENOUGH - insufficient balance; WRONG_PARAMETER;
//     /// RUNTIME_ERROR - a run-time error; TIMEOUT; WRONG_TX_FORMAT;
//     /// DUPLICCATE_SET_CODE - set code is duplicated unexpectedly;
//     /// UNKNOWN_ERROR
//     pub status_code: StatusCode,
//     /// a message descripting status_code
//     pub message: String,
//     /// return values for each Action
//     pub returns: Vec<String>,
//     /// for event functions
//     pub receipts: Vec<Receipt>,
// }
