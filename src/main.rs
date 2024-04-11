#![feature(error_in_core)]
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use led_game::LED;

#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use led_game::led_driver::led_driver;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());
    {
        *LED.lock().await = Some(Output::new(p.PIN_0, Level::Low));
    }

    spawner
        .spawn(led_driver())
        .expect("Fatal error spawning LED driver!");

    loop {
        {
            LED.lock()
                .await
                .as_mut()
                .expect("Internal Error: `LED` not initialized")
                .toggle();
        }
        Timer::after_millis(3_000).await;
    }
}
