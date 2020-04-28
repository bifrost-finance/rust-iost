use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    /// mode of concurrency; 0 - non-concurrent; 1 - concurrent
    pub mode: i32,
    /// the thread count of transaction concurrent execution
    pub thread: i32,
    /// indices of the transaction
    pub batch_index: Vec<i32>
}
