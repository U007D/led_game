#![allow(dead_code)]

use defmt::Format;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error, Format)]
pub enum Error {
    #[error("Internal Error: Invalid `Message` deserialization value ({0}) received!")]
    InvalidMessageDeserializationValue(u8),
}