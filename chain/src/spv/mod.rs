pub use self::{block::*, head::*, tx::*, verify::*};

pub mod block;
pub mod head;
pub mod tx;
pub mod verify;

pub const VOTE_INTERVAL: i64 = 1200;
pub const VERIFIER_NUM: usize = 17;
