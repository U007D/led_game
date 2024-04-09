mod numeric_led;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{
    PIN_0, PIN_1, PIN_10, PIN_11, PIN_12, PIN_2, PIN_3, PIN_4, PIN_5, PIN_6, PIN_7, PIN_8, PIN_9,
};
use embassy_time::Timer;

use crate::error::digit::Result;
use numeric_led::NumericLed;

pub struct Game<'d> {
    led: Output<'d, PIN_0>,
    four_digit_led: NumericLed<
        'd,
        PIN_1,
        PIN_2,
        PIN_3,
        PIN_4,
        PIN_5,
        PIN_6,
        PIN_7,
        PIN_8,
        PIN_9,
        PIN_10,
        PIN_11,
        PIN_12,
    >,
}

impl Game<'_> {
    pub fn new() -> Self {
        let periphs = embassy_rp::init(Default::default());
        Self {
            led: Output::new(periphs.PIN_0, Level::Low),
            four_digit_led: NumericLed::builder()
                .dig_1(periphs.PIN_1)
                .dig_2(periphs.PIN_2)
                .dig_3(periphs.PIN_3)
                .dig_4(periphs.PIN_4)
                .a(periphs.PIN_5)
                .b(periphs.PIN_6)
                .c(periphs.PIN_7)
                .d(periphs.PIN_8)
                .e(periphs.PIN_9)
                .f(periphs.PIN_10)
                .g(periphs.PIN_11)
                .dp(periphs.PIN_12)
                .build(),
        }
    }

    pub fn toggle_led(&mut self) -> &mut Self {
        self.led.toggle();
        self
    }

    pub async fn run(&mut self) -> Result<!> {
        loop {
            defmt::info!("Blink");

            self.led.set_high();
            self.four_digit_led.set_value(42);
            Timer::after_millis(500).await;

            self.led.set_low();
            self.four_digit_led.set_value(2468);
            Timer::after_millis(500).await;
        }
    }
}
