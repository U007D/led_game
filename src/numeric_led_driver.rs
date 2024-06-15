mod numeric_led;

use core::sync::atomic::Ordering;

use embassy_time::Duration;

use crate::SCORE;
pub use numeric_led::{DecimalSeparator, NumericLed, NumericLedPins};

// At the time of writing, `embassy-rs` does not support tasks with generic parameters.  The generic
// parameter arises from the `Output<'_, T>` type.  So to work around this issue, we provide the pin
// instances required by `NumericLed` without the accompanying borrow required by `Output`.  The
// pins are configured as `Output<'_, T>` pins within this task.
#[embassy_executor::task]
pub async fn numeric_led_driver(numeric_led_pins: NumericLedPins, persistence: Duration) -> ! {
    let mut numeric_led = NumericLed::from(numeric_led_pins);
    loop {
        let score = SCORE.load(Ordering::Relaxed);
        numeric_led.set(score, persistence).await;
    }
}
