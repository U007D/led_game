mod score_state;

use crate::{Message, SCORE_CHANNEL, SCORE};
use core::sync::atomic::Ordering;
use embassy_time::{Duration, Instant, Timer};
use score_state::ScoreMode;

#[embassy_executor::task]
pub async fn score_driver(update_period: Duration) {
    let mut start_time = Instant::now();
    let mut mode = ScoreMode::default();
    loop {
        let msg = SCORE_CHANNEL.receive().await;
        use Message as M;
        use ScoreMode as Sm;
        match (mode, msg) {
            // `ScoreState::Stopped` message handlers
            (Sm::Stopped, M::StartTimer) => {
                mode = Sm::Running;
                start_time = Instant::now();
            }
            (Sm::Stopped, _) => {}

            // `ScoreState::Running` message handlers
            (Sm::Running, M::StopTimer) => {
                update_score(start_time);
                mode = Sm::Stopped;
            }
            (Sm::Running, _) => {}
        }
        Timer::after(update_period).await;
    }
}

#[inline(always)]
fn update_score(start_time: Instant) {
    let duration = Instant::now() - start_time;
    let millis = u16::try_from(duration.as_millis()).unwrap_or(u16::MAX);

    SCORE.store(millis, Ordering::Relaxed);
}
