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
    #[error(
        "Error spawning new {1} task. (increase `#[embassy_executor::task(pool_size = n)]`)? {0:?}"
    )]
    TaskSpawn(embassy_executor::SpawnError, &'static str),
}

impl From<(embassy_executor::SpawnError, &'static str)> for Error {
    fn from((error, task_name): (embassy_executor::SpawnError, &'static str)) -> Self {
        Self::TaskSpawn(error, task_name)
    }
}
