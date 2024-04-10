use core::{char::TryFromCharError, num::TryFromIntError};

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
    A,
    B,
    C,
    D,
    E,
    F,
}

impl TryFrom<char> for LedDigit {
    type Error = TryFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const U8_OVERFLOW: char = '\u{0100}';

        let value = value.to_ascii_lowercase();

        use LedDigit as D;
        match value {
            'a' => Ok(D::A),
            'b' => Ok(D::B),
            'c' => Ok(D::C),
            'd' => Ok(D::D),
            'e' => Ok(D::E),
            'f' => Ok(D::F),
            // Unfortunately, `core::char::TryFromCharError` is not directly constructable (it
            // does not have a constructor, has private fields and is sealed); this workaround
            // manufactures one.
            _ => Err(u8::try_from(U8_OVERFLOW).unwrap_err()),
        }
    }
}

impl TryFrom<u16> for LedDigit {
    type Error = TryFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        const U8_OVERFLOW: u16 = 256;

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
            // Unfortunately, `core::num::TryFromIntError` is not directly constructable (it
            // does not have a constructor, has private fields and is sealed); this workaround
            // manufactures one.
            _ => Err(u8::try_from(U8_OVERFLOW).unwrap_err()),
        }
    }
}
