// #[cfg(feature = "std")]

// use base64;
// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};
// use sha3::{Digest, Sha3_256};
//
// use alloc::collections::BTreeMap;
// use alloc::format;
// use alloc::string::String;
// use alloc::vec::Vec;
// use keys::algorithm;
//
// use crate::Error::IOSTBlockVerifyError;
// use crate::Result;
// use crate::SerializeData;
// use crate::spv::{Head, Sign, Tx};
// use crate::TxReceipt;
//
// #[derive(Debug)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// pub struct Block {
//     /// block hash
//     pub head: Head,
//     /// block version number
//     pub sign: Sign,
//     /// public key of the block producer
//     pub receipts: Vec<TxReceipt>,
//     /// time of block production
//     pub txs: Vec<Tx>,
//     /// transaction number in the block
//     #[cfg(feature = "std")]
//     #[serde(rename = "txHashes")]
//     pub tx_hashes: Vec<String>,
//
//     #[cfg(feature = "std")]
//     #[serde(rename = "receiptHashes")]
//     pub receipt_hashes: Vec<String>,
//
//     #[cfg(feature = "std")]
//     #[serde(rename = "blockType")]
//     pub block_type: String,
// }
//
// impl Block {
//     pub(crate) fn verify_self(&self) -> Result<()> {
//         let ed25519 = algorithm::new(algorithm::ED25519);
//         let sign = base64::decode(self.sign.sig.as_str()).unwrap();
//         // #[cfg(feature = "std")]
//         // let pub_key = self.head.witness.from_base58().unwrap();
//         let pub_key = vec![];
//         let hash = self.head.hash();
//
//         if !ed25519.verify(hash.as_slice(), pub_key.as_slice(), sign.as_slice()) {
//             return Err(IOSTBlockVerifyError(format!("The signature of block {} is wrong", "EsoovjyX7ViMkbSs2y8gniJ7WQW33m8jVM8zvuZMKRRU")));
//         }
//
//         if self.txs.len() != self.receipts.len() {
//             return Err(IOSTBlockVerifyError(format!("Tx len {} unmatch receipt len {}", self.txs.len(), self.receipts.len())));
//         }
//         Ok(())
//     }
// }