use core::num::TryFromIntError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LedDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<u16> for LedDigit {
    type Error = TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        // TODO: Constify once const `From<T>` stabilizes
        let u8_overflow = u16::from(u8::MAX).saturating_add(1);
        let value = value.into();

        use LedDigit as D;
        match value {
            0 => Ok(D::Zero),
            1 => Ok(D::One),
            2 => Ok(D::Two),
            3 => Ok(D::Three),
            4 => Ok(D::Four),
            5 => Ok(D::Five),
            6 => Ok(D::Six),
            7 => Ok(D::Seven),
            8 => Ok(D::Eight),
            9 => Ok(D::Nine),
            // Sadly, `core::convert::TryFromIntError` is not directly constructable; this
            // workaround manufactures one.
            _ => Err(<u8 as TryFrom<u16>>::try_from(u8_overflow).unwrap_err()),
        }
    }
}
