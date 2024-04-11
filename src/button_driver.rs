use crate::{CHANNEL, message::Message};
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn button_driver(button_pin: AnyPin, led_pin: AnyPin) {
    let mut button = Input::new(button_pin, Pull::Down);
    let mut led = Output::new(led_pin, Level::High);

    loop {
        button.wait_for_rising_edge().await;
        led.toggle();
        // CHANNEL.send(Message::Start).await;

        button.wait_for_falling_edge().await;
        // CHANNEL.send(Message::Stop).await;
        led.toggle();
    }
}
