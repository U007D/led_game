#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_0, Level::Low);
    let mut _d1 = Output::new(peripherals.PIN_1, Level::Low);
    let mut _a = Output::new(peripherals.PIN_5, Level::High);

    loop {
        defmt::info!("Blink");

        led.set_high();
        Timer::after_millis(500).await;

        led.set_low();
        Timer::after_millis(500).await;
    }
}
