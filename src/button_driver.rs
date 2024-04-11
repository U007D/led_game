use crate::{CHANNEL, message::Message};
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use embassy_time::Timer;

#[embassy_executor::task(pool_size = 2)]
pub async fn button_driver(button_pin: AnyPin, led_pin: AnyPin) {
    // let mut button = Input::new(button_pin, Pull::Up);
    let mut led = Output::new(led_pin, Level::High);

    loop {
        Timer::after_millis(500).await;
        // button.wait_for_rising_edge().await;
        led.set_low();
        CHANNEL.send(Message::Start).await;
        Timer::after_millis(500).await;
        // button.wait_for_falling_edge().await;
        led.set_high();
        CHANNEL.send(Message::Stop).await;
    }
}
