use crate::message::ErrorMessage;

#[derive(Debug)]
pub enum Error {
    ///Error request message
    Reqwest(reqwest::Error),
    ///Error response message
    ErrorMessage(ErrorMessage)
}
