use crate::{message::NowPlayingLedMsg as Np, NOW_PLAYING_LED_CHANNEL};
use embassy_rp::gpio::{AnyPin, Level, Output};

#[embassy_executor::task]
pub async fn now_playing_led_driver(led_pin: AnyPin) -> ! {
    let mut led = Output::new(led_pin, Level::Low);
    loop {
        let message = NOW_PLAYING_LED_CHANNEL.receive().await;

        match message {
            Np::On => {
                led.set_high();
            }
            Np::Off => {
                led.set_low();
            }
        }
    }
}
