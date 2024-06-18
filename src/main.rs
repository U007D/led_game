#![feature(never_type)]
#![no_std]
#![no_main]

use defmt::error;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::{Duration, Timer};

use led_game::{
    button_driver,
    error::Result,
    game_old::{self, Game},
    game_loop, now_playing_led_driver,
    numeric_led_driver::{numeric_led_driver, NumericLedPins},
    score_driver, GAME_CHANNEL, LED_DISPLAY_PERSISTENCE_DELAY, MAX_WAITABLE_DURATION,
    NOW_PLAYING_LED_CHANNEL, NUMERIC_LED_CHANNEL, SCORE_CHANNEL, SCORE_DRIVER_UPDATE_PERIOD,
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
    let game = Game::new(
        spawner,
        GAME_CHANNEL.receiver(),
        NOW_PLAYING_LED_CHANNEL.sender(),
        NUMERIC_LED_CHANNEL.sender(),
        SCORE_CHANNEL.sender(),
    );
    game_old::run().await
}
