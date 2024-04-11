use crate::LED;
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn led_driver() {
    loop {
        {
            LED.lock()
                .await
                .as_mut()
                .expect("Internal Error: `LED` not initialized")
                .toggle();
        }
        Timer::after_millis(700).await;
    }
}
