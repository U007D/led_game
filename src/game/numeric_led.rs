mod digit_panel;
mod encoded_led_digit;

use core::cmp::min;
use embassy_rp::gpio::{Level, Output, Pin};
use typed_builder::TypedBuilder;

pub use {
    digit_panel::Panel,
    encoded_led_digit::{EncodedLedDigit, LedDigit},
};

#[derive(TypedBuilder)]
pub struct NumericLed<
    'd,
    TDig1: Pin,
    TDig2: Pin,
    TDig3: Pin,
    TDig4: Pin,
    TA: Pin,
    TB: Pin,
    TC: Pin,
    TD: Pin,
    TE: Pin,
    TF: Pin,
    TG: Pin,
    TDp: Pin,
> {
    #[builder(setter(transform = |pin: TDig1| Output::new(pin, Level::High)))]
    dig_1: Output<'d, TDig1>,
    #[builder(setter(transform = |pin: TDig2| Output::new(pin, Level::High)))]
    dig_2: Output<'d, TDig2>,
    #[builder(setter(transform = |pin: TDig3| Output::new(pin, Level::High)))]
    dig_3: Output<'d, TDig3>,
    #[builder(setter(transform = |pin: TDig4| Output::new(pin, Level::High)))]
    dig_4: Output<'d, TDig4>,
    #[builder(setter(transform = |pin: TA| Output::new(pin, Level::Low)))]
    a: Output<'d, TA>,
    #[builder(setter(transform = |pin: TB| Output::new(pin, Level::Low)))]
    b: Output<'d, TB>,
    #[builder(setter(transform = |pin: TC| Output::new(pin, Level::Low)))]
    c: Output<'d, TC>,
    #[builder(setter(transform = |pin: TD| Output::new(pin, Level::Low)))]
    d: Output<'d, TD>,
    #[builder(setter(transform = |pin: TE| Output::new(pin, Level::Low)))]
    e: Output<'d, TE>,
    #[builder(setter(transform = |pin: TF| Output::new(pin, Level::Low)))]
    f: Output<'d, TF>,
    #[builder(setter(transform = |pin: TG| Output::new(pin, Level::Low)))]
    g: Output<'d, TG>,
    #[builder(setter(transform = |pin: TDp| Output::new(pin, Level::Low)))]
    dp: Output<'d, TDp>,
}

impl<
        'd,
        TDig1: Pin,
        TDig2: Pin,
        TDig3: Pin,
        TDig4: Pin,
        TA: Pin,
        TB: Pin,
        TC: Pin,
        TD: Pin,
        TE: Pin,
        TF: Pin,
        TG: Pin,
        TDp: Pin,
    > NumericLed<'d, TDig1, TDig2, TDig3, TDig4, TA, TB, TC, TD, TE, TF, TG, TDp>
{
    const DECIMAL_BASE: u16 = 10;
    const MAX_DISPLAYABLE_VALUE: u16 = 9999;
    const PANELS: [Panel; 4] = [Panel::One, Panel::Two, Panel::Three, Panel::Four];

    pub fn clear_value(&mut self) -> &mut Self {
        self.clear_digit(Panel::One);
        self.clear_digit(Panel::Two);
        self.clear_digit(Panel::Three);
        self.clear_digit(Panel::Four);
        self
    }

    pub fn set_value(&mut self, value: u16) -> &mut Self {
        // Clip value, if necessary
        let mut value = min(value, Self::MAX_DISPLAYABLE_VALUE);

        // Build and write `value`'s digits to individual LED panels starting from the ones column
        // (`Panel::Four`) to the 10,000's column (`Panel::One`)
        Self::PANELS.into_iter().rev().for_each(|panel| {
            // PANIC SAFETY //
            // Generating a base-10 (`DECIMAL_BASE`) `LedDigit` from a `u16` is always valid and
            // thus cannot fail.
            let digit = LedDigit::try_from(value % Self::DECIMAL_BASE).unwrap_or_else(|_err| {
                unreachable!(
                    "Internal error: Extraction of a base-10 digit from a valid value unexpectedly yielded a non-digit result."
                )
            });
            value /= Self::DECIMAL_BASE;
            self.set_digit(panel, digit);
        });
        self
    }

    #[inline(always)]
    fn clear_digit(&mut self, panel: Panel) -> &mut Self {
        self.enable_write(panel);
        self.a.set_low();
        self.b.set_low();
        self.c.set_low();
        self.d.set_low();
        self.e.set_low();
        self.f.set_low();
        self.g.set_low();
        self.dp.set_low();
        self.disable_write(panel);
        self
    }

    #[inline(always)]
    fn disable_write(&mut self, panel: Panel) -> &mut Self {
        use Panel as P;
        match panel {
            P::One => self.dig_1.set_high(),
            P::Two => self.dig_2.set_high(),
            P::Three => self.dig_3.set_high(),
            P::Four => self.dig_4.set_high(),
        }
        self
    }

    #[inline(always)]
    fn enable_write(&mut self, panel: Panel) -> &mut Self {
        use Panel as P;
        match panel {
            P::One => self.dig_1.set_low(),
            P::Two => self.dig_2.set_low(),
            P::Three => self.dig_3.set_low(),
            P::Four => self.dig_4.set_low(),
        }
        self
    }

    #[inline(always)]
    fn set_digit<E: Into<EncodedLedDigit>>(&mut self, panel: Panel, encoded_digit: E) -> &mut Self {
        self.enable_write(panel);
        let encoded_value = encoded_digit.into().pattern();
        self.a.set_level((encoded_value & (0x1 << 0) == 0).into());
        self.b.set_level((encoded_value & (0x1 << 1) == 0).into());
        self.c.set_level((encoded_value & (0x1 << 2) == 0).into());
        self.d.set_level((encoded_value & (0x1 << 3) == 0).into());
        self.e.set_level((encoded_value & (0x1 << 4) == 0).into());
        self.f.set_level((encoded_value & (0x1 << 5) == 0).into());
        self.g.set_level((encoded_value & (0x1 << 6) == 0).into());
        self.dp.set_level((encoded_value & (0x1 << 7) == 0).into());
        self.disable_write(panel);
        self
    }
}
