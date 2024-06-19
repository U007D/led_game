#![feature(never_type)]
#![no_std]
#![no_main]

use defmt::error;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::Timer;

use led_game::{
    button_driver,
    error::Result,
    // game_old::{self, Game},
    game_loop,
    now_playing_led_driver,
    numeric_led_driver::{numeric_led_driver, NumericLedPins},
    score_driver,
    LED_DISPLAY_PERSISTENCE_DELAY,
    MAX_WAITABLE_DURATION,
    SCORE_DRIVER_UPDATE_PERIOD,
};

#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let _error = inner_main(spawner).await.unwrap_err();
    error!("`led_game` exited with error.");
    loop {
        Timer::after(MAX_WAITABLE_DURATION).await;
    }
}

async fn inner_main(spawner: Spawner) -> Result<!> {
    let periphs = embassy_rp::init(Default::default());

    let led_display_pins = NumericLedPins::new(
        periphs.PIN_1,
        periphs.PIN_2,
        periphs.PIN_3,
        periphs.PIN_4,
        periphs.PIN_5,
        periphs.PIN_6,
        periphs.PIN_7,
        periphs.PIN_8,
        periphs.PIN_9,
        periphs.PIN_10,
        periphs.PIN_11,
        periphs.PIN_12,
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
        .spawn(now_playing_led_driver(periphs.PIN_0.degrade()))
        .map_err(|err| (err, "solo_led_driver"))?;

    spawner
        .spawn(button_driver(periphs.PIN_13.degrade()))
        .map_err(|err| (err, "button_driver"))?;

    game_loop().await
}
