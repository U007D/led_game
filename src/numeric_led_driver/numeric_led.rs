mod encoded_led_digit;
mod numeric_led_pins;
mod led_panel;

use core::cmp::min;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::{Duration, Timer};

use crate::numeric_led_driver::DecimalPos;
pub use {
    encoded_led_digit::{EncodedLedDigit, NumberLedDigit},
    numeric_led_pins::NumericLedPins,
    led_panel::LedPanel,
};

pub struct NumericLed<'d> {
    dig_1: Output<'d, AnyPin>,
    dig_2: Output<'d, AnyPin>,
    dig_3: Output<'d, AnyPin>,
    dig_4: Output<'d, AnyPin>,
    a: Output<'d, AnyPin>,
    b: Output<'d, AnyPin>,
    c: Output<'d, AnyPin>,
    d: Output<'d, AnyPin>,
    e: Output<'d, AnyPin>,
    f: Output<'d, AnyPin>,
    g: Output<'d, AnyPin>,
    dp: Output<'d, AnyPin>,
}

impl NumericLed<'_> {
    pub const MAX: u16 = 9999;
    const DECIMAL_BASE: u16 = 10;
    const PANEL: [LedPanel; 4] = [LedPanel::Four, LedPanel::Three, LedPanel::Two, LedPanel::One];

    pub fn new(
        dig_1: AnyPin,
        dig_2: AnyPin,
        dig_3: AnyPin,
        dig_4: AnyPin,
        a: AnyPin,
        b: AnyPin,
        c: AnyPin,
        d: AnyPin,
        e: AnyPin,
        f: AnyPin,
        g: AnyPin,
        dp: AnyPin,
    ) -> Self {
        Self {
            dig_1: Output::new(dig_1, Level::High),
            dig_2: Output::new(dig_2, Level::High),
            dig_3: Output::new(dig_3, Level::High),
            dig_4: Output::new(dig_4, Level::High),
            a: Output::new(a, Level::Low),
            b: Output::new(b, Level::Low),
            c: Output::new(c, Level::Low),
            d: Output::new(d, Level::Low),
            e: Output::new(e, Level::Low),
            f: Output::new(f, Level::Low),
            g: Output::new(g, Level::Low),
            dp: Output::new(dp, Level::Low),
        }
    }

    pub async fn set_digit<E: Into<EncodedLedDigit>>(
        &mut self,
        panel: LedPanel,
        encoded_digit: E,
        decimal_pos: DecimalPos,
        persistence: Duration,
    ) -> &mut Self {
        self.enable_write(panel);
        let encoding = encoded_digit.into().encoding();
        use EncodedLedDigit as Ed;
        self.a
            .set_level((encoding & (0x1 << Ed::A_BIT_POS) != 0).into());
        self.b
            .set_level((encoding & (0x1 << Ed::B_BIT_POS) != 0).into());
        self.c
            .set_level((encoding & (0x1 << Ed::C_BIT_POS) != 0).into());
        self.d
            .set_level((encoding & (0x1 << Ed::D_BIT_POS) != 0).into());
        self.e
            .set_level((encoding & (0x1 << Ed::E_BIT_POS) != 0).into());
        self.f
            .set_level((encoding & (0x1 << Ed::F_BIT_POS) != 0).into());
        self.g
            .set_level((encoding & (0x1 << Ed::G_BIT_POS) != 0).into());
        self.dp.set_level(
            ((encoding & (0x1 << Ed::DP_BIT_POS) != 0) || panel.has_dp(decimal_pos)).into(),
        );
        Timer::after(persistence).await;
        self.disable_write(panel);
        self
    }

    fn disable_write(&mut self, panel: LedPanel) -> &mut Self {
        use LedPanel as P;
        match panel {
            P::One => self.dig_1.set_high(),
            P::Two => self.dig_2.set_high(),
            P::Three => self.dig_3.set_high(),
            P::Four => self.dig_4.set_high(),
        }
        self
    }
    //
    // #[inline(always)]
    pub fn enable_write(&mut self, panel: LedPanel) -> &mut Self {
        use LedPanel as P;
        match panel {
            P::One => self.dig_1.set_low(),
            P::Two => self.dig_2.set_low(),
            P::Three => self.dig_3.set_low(),
            P::Four => self.dig_4.set_low(),
        }
        self
    }

    pub async fn set<V: Into<u16>>(
        &mut self,
        value: V,
        decimal_pos: DecimalPos,
        persistence: Duration,
    ) -> &mut Self {
        // Clip value to displayable range 0000-9999.
        let mut value = min(value.into(), Self::MAX);

        // Render each digit (with a delay for each)
        for panel in Self::PANEL.into_iter() {
            let digit = NumberLedDigit::try_from(value % Self::DECIMAL_BASE).unwrap_or_else(|_err| {
                unreachable!(
                    "Internal error: Extraction of a base-10 digit from a valid value unexpectedly yielded a non-digit result."
                )
            });
            value /= Self::DECIMAL_BASE;
            self.set_digit(panel, digit, decimal_pos, persistence).await;
        }
        self
    }
}

impl From<NumericLedPins> for NumericLed<'_> {
    fn from(p: NumericLedPins) -> Self {
        NumericLed::new(
            p.dig_1, p.dig_2, p.dig_3, p.dig_4, p.a, p.b, p.c, p.d, p.e, p.f, p.g, p.dp,
        )
    }
}
