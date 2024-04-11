mod decimal_pos;
mod numeric_led;

use core::sync::atomic::Ordering;

use embassy_time::Duration;

pub use crate::DECIMAL_POS;
use crate::SCORE;
pub use {decimal_pos::DecimalPos, numeric_led::NumericLed, numeric_led::NumericLedPins};

#[embassy_executor::task]
pub async fn numeric_led_driver(display_pins: NumericLedPins, persistence: Duration) {
    let mut led_display = NumericLed::from(display_pins);
    loop {
        let score = SCORE.load(Ordering::Relaxed);
        let decimal_pos = *DECIMAL_POS.lock().await;
        led_display.set(score, decimal_pos, persistence).await;
    }
}
