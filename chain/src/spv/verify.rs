use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::Error::*;
use crate::Result;

use super::{Block, Head, VERIFIER_NUM, VOTE_INTERVAL};

#[derive(Debug, Default)]
pub struct Verify {
    epoch_producer: BTreeMap<i64, Vec<String>>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct WitnessStatus {
    #[cfg(feature = "std")]
    #[serde(rename = "pendingList")]
    pub pending_list: Vec<String>,

    #[cfg(feature = "std")]
    #[serde(rename = "currentList")]
    pub current_list: Vec<String>,
}

impl Verify {
    #[cfg(feature = "std")]
    pub fn check_block(&self, block: &Block, block_list: Vec<Block>) -> Result<()> {
        #[cfg(feature = "std")]
        match check_witness(self, block, block_list) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[cfg(feature = "std")]
    pub fn update_epoch(&mut self, block: &Block, block_list: Vec<Block>) -> Result<()> {
        let head: Head = block.head.clone();
        let vote_block_number = head.number;
        if vote_block_number % VOTE_INTERVAL != 0 {
            return Err(IOSTUpdateEpochError(format!(
                "invalid spv start block {}",
                vote_block_number
            )));
        }
        match get_witness_status_from_block(block) {
            Some(w) => {
                if w.pending_list.len() != VERIFIER_NUM {
                    return Err(IOSTUpdateEpochError(format!(
                        "invalid pending list length {} at block {}",
                        w.pending_list.len(),
                        vote_block_number
                    )));
                }

                #[cfg(feature = "std")]
                match check_witness(self, block, block_list) {
                    Ok(_) => {
                        self.epoch_producer
                            .insert(vote_block_number, w.pending_list.clone());
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            None => Err(IOSTUpdateEpochError(format!(
                "vote_producer.iost/stat receipt not found at block {} ,hash: {:?}",
                head.number,
                head.hash()
            ))),
        }
    }
}

#[cfg(feature = "std")]
pub fn init(block: &Block) -> Result<Verify> {
    let head: Head = block.head.clone();

    let block_number: i64 = head.number;

    if block_number % VOTE_INTERVAL != 0 {
        return Err(InvalidSPVStartBlock(block_number));
    }

    match get_witness_status_from_block(block) {
        Some(witness_status) => {
            if witness_status.pending_list.len() != VERIFIER_NUM {
                return Err(IOSTBlockError());
            }

            let mut v = Verify {
                epoch_producer: BTreeMap::new(),
            };
            v.epoch_producer
                .insert(block_number, witness_status.pending_list.clone());

            return Ok(v);
        }
        None => {
            return Err(IOSTBlockError());
        }
    }
}

#[cfg(feature = "std")]
pub fn get_witness_status_from_block(block: &Block) -> Option<WitnessStatus> {
    for tx_receipt in block.receipts.iter() {
        for receipt in tx_receipt.receipts.iter() {
            if receipt.func_name == "vote_producer.iost/stat" {
                #[cfg(feature = "std")]
                match serde_json::from_str(&receipt.content) {
                    Ok(ws) => {
                        return Some(ws);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    }
    None
}

#[cfg(feature = "std")]
pub fn check_witness(v: &Verify, block: &Block, witness_blocks: Vec<Block>) -> Result<()> {
    if let Err(_) = block.verify_self() {
        return Err(IOSTInvalidBlockSignature());
    }

    for b in witness_blocks.iter() {
        if let Err(_) = b.verify_self() {
            return Err(IOSTInvalidBlockSignature());
        }
    }

    let block_number: i64 = block.head.number;
    let current_epoch_start_block = if block_number % VOTE_INTERVAL == 0 {
        block_number - VOTE_INTERVAL
    } else {
        block_number / VOTE_INTERVAL * VOTE_INTERVAL
    };

    match v.epoch_producer.get(&current_epoch_start_block) {
        Some(pending_list) => {
            let mut valid_witness_count = 0;
            let mut valid_witness: BTreeMap<String, bool> = BTreeMap::new();

            let mut parent_hash = block.head.hash();
            let mut parent_block_number = block.head.number;

            for b in witness_blocks.iter() {
                // let block_parent_hash = &b.head.parent_hash;
                if parent_hash.as_slice() != b.head.parent_hash.as_slice() {
                    return Err(IOSTBlockWitnessError(format!(
                        "invalid block hash at block {}",
                        b.head.number
                    )));
                }
                if parent_block_number + 1 != b.head.number {
                    return Err(IOSTBlockWitnessError(format!(
                        "invalid block number at block {}",
                        b.head.number
                    )));
                }

                match valid_witness.get(&b.head.witness) {
                    None => {
                        for produce in pending_list.iter() {
                            if produce.eq(&b.head.witness) {
                                valid_witness.insert(produce.to_string(), true);
                                valid_witness_count = valid_witness_count + 1;
                                break;
                            }
                        }
                    }
                    _ => {}
                }
                parent_block_number = b.head.number;
                parent_hash = b.head.hash();
            }
            if valid_witness_count < 12 {
                return Err(IOSTBlockWitnessError(format!(
                    "valid witness not enough {}",
                    valid_witness_count
                )));
            }
            Ok(())
        }
        None => Err(IOSTBlockWitnessError(format!(
            "cannot update producer list at block {}: cannot find producer info of previous epoch",
            block_number
        ))),
    }
}
