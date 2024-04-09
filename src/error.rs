#![allow(dead_code)]
pub mod digit;

use defmt::Format;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error, Format)]
pub enum Error {
    #[error(transparent)]
    Digit(#[from] digit::Error),
}