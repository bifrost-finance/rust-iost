use crate::message::ErrorMessage;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    JsonReadError,
    Reqwest(reqwest::Error),
    ErrorMessage(ErrorMessage),
}
