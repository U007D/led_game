#![feature(never_type)]
#![no_std]
#![no_main]

use defmt::error;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};

use led_game::{
    error::Result,
    numeric_led_driver::{numeric_led_driver, NumericLedPins},
};

use led_game::MAX_WAITABLE_DURATION;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _error = inner_main(_spawner).await.unwrap_err();
    error!("`led_game` exited with error.");
    loop {
        Timer::after(MAX_WAITABLE_DURATION).await;
    }
}

async fn inner_main<'pin>(_spawner: Spawner) -> Result<!> {
    let periphs = embassy_rp::init(Default::default());

    // let led_display_pins = NumericLedPins::new(
    //     periphs.PIN_1,
    //     periphs.PIN_2,
    //     periphs.PIN_3,
    //     periphs.PIN_4,
    //     periphs.PIN_5,
    //     periphs.PIN_6,
    //     periphs.PIN_7,
    //     periphs.PIN_8,
    //     periphs.PIN_9,
    //     periphs.PIN_10,
    //     periphs.PIN_11,
    //     periphs.PIN_12,
    // );

    loop {
    }
}
