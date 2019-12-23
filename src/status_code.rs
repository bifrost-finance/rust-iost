use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize,Debug)]
pub enum StatusCode {
    SUCCESS,
    GAS_RUN_OUT,
    BALANCE_NOT_ENOUGH,
    WRONG_PARAMETER,
    RUNTIME_ERROR,
    TIMEOUT,
    WRONG_TX_FORMAT,
    DUPLICATE_SET_CODE,
    UNKNOWN_ERROR
}