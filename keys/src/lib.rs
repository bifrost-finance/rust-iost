#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod algorithm;
pub mod error;
pub mod secret;
pub mod signature;

mod base58;
mod constant;
mod network;

use error::Result;
