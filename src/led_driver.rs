use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn led_driver(led_pin: AnyPin) {
    let mut led = Output::new(led_pin, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
