mod game_mode;

use core::sync::atomic::Ordering;
use embassy_futures::select::{select, select_array, Either};
use embassy_time::Timer;
use game_mode::GameMode;

use crate::message::NowPlayingLedMsg;
use crate::{message, GAME_CHANNEL, NOW_PLAYING_LED_CHANNEL, SCORE_CHANNEL, SCORE};

pub async fn game_loop() -> ! {
    let mut mode = GameMode::Stopped;

    loop {
        let msg = GAME_CHANNEL.receive().await;

        use message::{GameMsg as GMsg, NowPlayingLedMsg as NpMsg, ScoreMsg as SMsg};
        use GameMode as GMode;
        match (mode, msg) {
            // GameMode::Stopped message handlers
            (GMode::Stopped, GMsg::ButtonDown) => {
                SCORE.store(0, Ordering::Relaxed);
            }

            (GMode::Stopped, GMsg::ButtonUp) => {
                NOW_PLAYING_LED_CHANNEL.send(NpMsg::On).await;
                Timer::after_secs(3).await;
                SCORE_CHANNEL.send(SMsg::Start).await;
                // let msg_res = GAME_CHANNEL.try_receive();
                // match msg_res {
                //     // User pressed button too early; gets max score
                //     Ok(_early_msg) => {
                //         SCORE.store(NumericLed::MAX, Ordering::Relaxed);
                //         mode = GameMode::Stopped;
                //     }
                //     // No button press yet--game on
                //     Err(_) => {

                mode = GameMode::Timing;
                //     }
                // }
            }

            // `GameMode::Timing` message handlers
            (GMode::Timing, GMsg::ButtonDown) => {
                SCORE_CHANNEL.send(SMsg::Stop).await;
                NOW_PLAYING_LED_CHANNEL.send(NpMsg::Off).await;
            }

            // Hold off on setting `GameMode::Stopped` until the button is released
            // (otherwise state would conflate with `(GMode::Stopped, BMsg::Up)`)
            (GMode::Timing, GMsg::ButtonUp) => {
                mode = GameMode::Stopped;
            }

            (_, GMsg::ScoreOverflow) => {
                SCORE_CHANNEL.send(SMsg::Stop).await;
                NOW_PLAYING_LED_CHANNEL.send(NpMsg::Off).await;
                mode = GameMode::Stopped;
            }
        }
    }
}
