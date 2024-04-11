use crate::{Message, SOLO_LED_CHANNEL};
use embassy_rp::gpio::{AnyPin, Level, Output};

#[embassy_executor::task]
pub async fn solo_led_driver(led_pin: AnyPin) {
    let mut led = Output::new(led_pin, Level::Low);
    loop {
        let message = SOLO_LED_CHANNEL.receive().await;

        use Message as M;
        match message {
            M::SoloLedOn => {
                led.set_high();
            }
            M::SoloLedOff => {
                led.set_low();
            }
            _ => {}
        }
    }
}
