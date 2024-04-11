mod number_led_digit;

pub use number_led_digit::NumberLedDigit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodedLedDigit(NumberLedDigit);

impl EncodedLedDigit {
    pub const A_BIT_POS: usize = 0;
    pub const B_BIT_POS: usize = 1;
    pub const C_BIT_POS: usize = 2;
    pub const D_BIT_POS: usize = 3;
    pub const E_BIT_POS: usize = 4;
    pub const F_BIT_POS: usize = 5;
    pub const G_BIT_POS: usize = 6;
    pub const DP_BIT_POS: usize = 7;

    pub fn encoding(&self) -> u8 {
        use NumberLedDigit as D;
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
            // Hexadecimal digit encodings
            D::A => 0b0111_0111,
            D::B => 0b0111_1100,
            D::C => 0b0011_1001,
            D::D => 0b0101_1110,
            D::E => 0b0111_1001,
            D::F => 0b0111_0001,
        }
    }
}

impl From<NumberLedDigit> for EncodedLedDigit {
    fn from(digit: NumberLedDigit) -> Self {
        Self(digit)
    }
}
