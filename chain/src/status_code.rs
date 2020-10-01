#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum StatusCode {
    SUCCESS,
    GAS_RUN_OUT,
    BALANCE_NOT_ENOUGH,
    WRONG_PARAMETER,
    RUNTIME_ERROR,
    TIMEOUT,
    WRONG_TX_FORMAT,
    DUPLICATE_SET_CODE,
    UNKNOWN_ERROR,
}
