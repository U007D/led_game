#![feature(error_in_core)]
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::Timer;

#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use led_game::{led_driver, button_driver, Message};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    spawner
        .spawn(led_driver(p.PIN_1.degrade()))
        .expect("Fatal error spawning LED driver!");

    spawner
        .spawn(button_driver(p.PIN_13.degrade(), p.PIN_0.degrade()))
        .expect("Fatal error spawning LED driver!");

    loop {
        Timer::after_millis(3_000).await;
    }
}
