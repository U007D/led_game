mod game_mode;

use core::sync::atomic::Ordering;
use embassy_time::Timer;
use game_mode::GameMode;

use crate::numeric_led_driver::NumericLed;
use crate::{Message, GAME_CHANNEL, SCORE_CHANNEL, SOLO_LED_CHANNEL, SCORE};

pub async fn game_loop() -> ! {
    let mut mode = GameMode::default();

    loop {
        let msg = GAME_CHANNEL.receive().await;
        use GameMode as Gm;
        use Message as M;

        match (mode, msg) {
            // GameMode::Stopped message handlers
            (Gm::Stopped, M::ButtonDown) => {
                SOLO_LED_CHANNEL.send(M::SoloLedOn).await;
                SCORE.store(0, Ordering::Relaxed);
            }
            (Gm::Stopped, Message::ButtonUp) => {
                GAME_CHANNEL.send(M::SoloLedOff).await;
                Timer::after_secs(3).await;
                // let msg_res = GAME_CHANNEL.try_receive();
                // match msg_res {
                //     // User pressed button too early; gets max score
                //     Ok(_early_msg) => {
                //         SCORE.store(NumericLed::MAX, Ordering::Relaxed);
                //         mode = GameMode::Stopped;
                //     }
                //     // No button press yet--game on
                //     Err(_) => {
                        SOLO_LED_CHANNEL.send(M::SoloLedOn).await;
                        SCORE_CHANNEL.send(M::StartTimer).await;
                        mode = GameMode::Timing;
                //     }
                // }
            }
            (Gm::Stopped, _) => {}

            // GameMode::Timing message handlers
            (Gm::Timing, Message::ButtonDown) => {
                SCORE_CHANNEL.send(M::StopTimer).await;
                SOLO_LED_CHANNEL.send(M::SoloLedOff).await;
                mode = GameMode::Stopped;
            }
            (Gm::Timing, _) => {}
        }
    }
}
