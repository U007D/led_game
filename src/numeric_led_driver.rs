mod numeric_led;

use embassy_time::Duration;

pub use numeric_led::{DecimalSeparator, NumericLed, NumericLedPins};

pub async fn numeric_led_driver(score: u16, numeric_led_pins: NumericLedPins, persistence: Duration) -> ! {
    let mut numeric_led = NumericLed::from(numeric_led_pins);
    loop {
        numeric_led.set(score, persistence).await;
    }
}
