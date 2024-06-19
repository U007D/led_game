mod score_state;

use embassy_time::Duration;
use score_state::ScoreState;

#[embassy_executor::task]
pub async fn score_driver(update_period: Duration) -> ! {
    let mut score_state = ScoreState::Stopped;
    loop {
        score_state.advance(update_period).await
    }
}
