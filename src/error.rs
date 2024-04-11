#![allow(dead_code)]
pub mod digit;
pub mod hex_digit;
pub mod message;

use defmt::Format;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error, Format)]
pub enum Error {
    #[error(transparent)]
    Digit(#[from] digit::Error),
    #[error(transparent)]
    HexDigit(#[from] hex_digit::Error),
    #[error(transparent)]
    Message(#[from] message::Error),
}