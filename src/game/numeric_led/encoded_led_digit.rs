mod led_digit;

pub use led_digit::LedDigit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodedLedDigit(LedDigit);

impl EncodedLedDigit {
    pub fn pattern(&self) -> u8 {
        use LedDigit as D;
        match self.0 {
            D::Zero => 0b0011_1111,
            D::One => 0b0000_0110,
            D::Two => 0b0101_1011,
            D::Three => 0b0100_1111,
            D::Four => 0b0110_0110,
            D::Five => 0b0110_1101,
            D::Six => 0b0111_1101,
            D::Seven => 0b0000_0111,
            D::Eight => 0b0111_1111,
            D::Nine => 0b0110_1111,
            D::A => 0b0111_0111,
            D::B => 0b0111_1100,
            D::C => 0b0011_1001,
            D::D => 0b0101_1110,
            D::E => 0b0111_1001,
            D::F => 0b0111_0001,
        }
    }
}

impl From<LedDigit> for EncodedLedDigit {
    fn from(digit: LedDigit) -> Self {
        Self(digit)
    }
}
