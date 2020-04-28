use crate::tx_receipt::TxReceipt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxResponse {
    /// Hash of transaction
    pub hash: String,
    /// The receipt of the transaction pre executed by the RPC node requires the RPC node to turn on the pre execution switch to return this field
    pub pre_tx_receipt: TxReceipt
}
