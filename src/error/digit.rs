use defmt::Format;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error, Format)]
pub enum Error {
    #[error("Error attempting to convert a invalid value to a digit.  The value must be between 0 and 9, inclusive). ({0})")]
    NonDigit(#[from] core::num::TryFromIntError),
}