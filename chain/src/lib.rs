#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod abi;
pub mod action;
pub mod amount_limit;
pub mod block;
pub mod bytes;

mod chain_test;

pub mod error;
pub mod fixed;
pub mod frozen_balance;
pub mod group;
pub mod info;
pub mod item;
pub mod key_field;
pub mod message;
pub mod names;
pub mod net_work_info;
pub mod permission;
pub mod pledge_info;
pub mod ram_info;
pub mod receipts;
pub mod signature;
pub mod status;
pub mod status_code;
pub mod test;
pub mod time_point;
pub mod transaction;
pub mod tx;
pub mod tx_receipt;
pub mod tx_response;
pub mod unsigned_int;
pub mod vote_info;

mod get_chain_info;

pub use iost_derive::*;

pub use self::{
    abi::*, action::*, amount_limit::*, block::*, bytes::*, error::*, fixed::*, frozen_balance::*,
    group::*, info::*, item::*, key_field::*, message::*, names::*, net_work_info::*,
    permission::*, pledge_info::*, ram_info::*, receipts::*, signature::*, status::*,
    status_code::*, transaction::*, tx::*, tx_receipt::*, tx_response::*, unsigned_int::*,
    vote_info::*,
};

use alloc::vec;
use alloc::vec::Vec;

pub trait SerializeData: Write + NumberBytes {
    fn to_serialize_data(&self) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0)
            .map_err(crate::Error::BytesWriteError)?;
        Ok(data.to_vec())
    }
}
