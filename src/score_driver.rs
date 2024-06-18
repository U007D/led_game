mod score_mode;

use crate::{message::ScoreMsg as SMsg, SCORE_CHANNEL, MAX_GAME_SCORE_MS, MAX_WAITABLE_DURATION, SCORE, GAME_CHANNEL};
use core::sync::atomic::Ordering;
use embassy_time::{with_timeout, Duration, Instant};
use score_mode::ScoreMode;
use crate::message::GameMsg;

#[embassy_executor::task]
pub async fn score_driver(active_update_period: Duration) -> ! {
    let mut score_mode = ScoreMode::Stopped;
    let mut update_period = MAX_WAITABLE_DURATION;
    loop {
        let start_time = Instant::now();
        loop {
            // Get next message or wait for `update_period`, whichever occurs first
            let operation = with_timeout(update_period, SCORE_CHANNEL.receive()).await;

            use ScoreMode as SMode;
            match operation {
                // Message arrived.  Process it
                Ok(msg) => {
                    match (score_mode, msg) {
                        // Go!
                        (SMode::Stopped, SMsg::Start) => {
                            score_mode = SMode::Running;
                            update_period = active_update_period;
                            break;
                        }

                        // Stop!
                        (SMode::Running, SMsg::Stop) => {
                            update_score(start_time);
                            score_mode = SMode::Stopped;
                            update_period = MAX_WAITABLE_DURATION;
                        }

                        // Ignore invalid message combinations
                        (SMode::Stopped, SMsg::Stop) | (SMode::Running, SMsg::Start) => {}
                    }
                }

                // Timeout expired. No problem, just fall through to (conditionally) update score
                // and continue waiting for a message
                Err(_) => {}
            }

            (score_mode == SMode::Running).then(|| {
                let elapsed_millis = update_score(start_time);
                (elapsed_millis > MAX_GAME_SCORE_MS)
                    .then(|| async { GAME_CHANNEL.send(GameMsg::ScoreOverflow).await } )
            });
        }
    }
}

#[inline(always)]
fn update_score(start_time: Instant) -> u16 {
    let duration = Instant::now().saturating_duration_since(start_time);
    let millis = u16::try_from(duration.as_millis()).unwrap_or(u16::MAX);
    SCORE.store(millis, Ordering::Relaxed);
    millis
}
