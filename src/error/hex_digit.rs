use defmt::Format;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error, Format)]
pub enum Error {
    #[error(
        "Error attempting to convert a invalid value to a hex digit.  The value must be between \
             'a' and 'f', inclusive (case-insensitive). ({0})"
    )]
    // `defmt` does not `impl Format for core::char::TryFromCharError`.  I tried forking `defmt` and
    // implementing, but per https://github.com/knurling-rs/defmt/issues/426, it's not possible to
    // have more than one distinct build of `defmt` in a build.  Because of this I also can't
    // meaningfully test the changes I made before upstreaming them, so this error simply returns
    // `core::num::TryFromIntError` instead.  No real information is being lost as both `Error`s
    // only carry `()` as their payload and `Error::NonDigit` is in the `hex_digit` error module.
    // TODO: Change payload once `defmt` implements `Format for core::char::TryFromCharError`.
    NonDigit(#[from] core::num::TryFromIntError),
}

// Per `defmt` note above, allow application code to still work with `TryFromCharError` as
// appropriate, and switch to `TryFromIntError` at the last moment.
// TODO: Remove this impl once `defmt` implements `Format for core::char::TryFromCharError`.
impl From<core::char::TryFromCharError> for Error {
    fn from(_value: core::char::TryFromCharError) -> Self {
        const U8_OVERFLOW: u16 = 256;
        // Unfortunately, `core::num::TryFromIntError` is not directly constructable (it
        // does not have a constructor, has private fields and is sealed); this workaround
        // manufactures one.
        Self::NonDigit(u8::try_from(U8_OVERFLOW).unwrap_err())
    }
}