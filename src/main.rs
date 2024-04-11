#![feature(error_in_core, never_type)]
#![no_std]
#![no_main]

use defmt::error;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::{Duration, Timer};

use led_game::{button_driver, error::Result, game_loop, numeric_led_driver::{numeric_led_driver, NumericLedPins}, score_driver, solo_led_driver};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

// Define maximum waitable `Duration` (`Timer::after()`).  Emperically derived.
const MAX_DURATION: Duration = Duration::from_secs(1 << 45 -1);
const LED_DISPLAY_PERSISTENCE_DELAY: Duration = Duration::from_millis(1);
const SCORE_DRIVER_UPDATE_PERIOD: Duration = Duration::from_millis(1);

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let _error = inner_main(spawner).await.unwrap_err();
    error!("`led_game` exited with error.");
    loop {
        Timer::after(MAX_DURATION).await;
    }
}

async fn inner_main(spawner: Spawner) -> Result<!> {
    let p = embassy_rp::init(Default::default());
    let led_display_pins = NumericLedPins::new(
        p.PIN_1, p.PIN_2, p.PIN_3, p.PIN_4, p.PIN_5, p.PIN_6, p.PIN_7, p.PIN_8, p.PIN_9, p.PIN_10,
        p.PIN_11, p.PIN_12,
    );

    spawner
        .spawn(numeric_led_driver(
            led_display_pins,
            LED_DISPLAY_PERSISTENCE_DELAY,
        ))
        .map_err(|err| (err, "numeric_led_driver"))?;

    spawner
        .spawn(score_driver(SCORE_DRIVER_UPDATE_PERIOD))
        .map_err(|err| (err, "score _driver"))?;

    spawner
        .spawn(solo_led_driver(p.PIN_0.degrade()))
        .map_err(|err| (err, "solo_led_driver"))?;

    spawner
        .spawn(button_driver(p.PIN_13.degrade()))
        .map_err(|err| (err, "button_driver"))?;

    game_loop().await
}
