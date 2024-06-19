use crate::{
    message::{GameMsg, ScoreMsg},
    GAME_CHANNEL, MAX_GAME_SCORE_MS, SCORE, SCORE_CHANNEL,
};
use core::{ops::ControlFlow, sync::atomic::Ordering};
use embassy_time::{with_timeout, Duration, Instant};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScoreState {
    #[default]
    Stopped,
    Running(Instant),
}

impl ScoreState {
    pub async fn advance(&mut self, update_period: Duration) {
        match *self {
            ScoreState::Stopped => loop {
                let msg = SCORE_CHANNEL.receive().await;

                match msg {
                    ScoreMsg::Start => {
                        *self = ScoreState::Running(Instant::now());
                        break;
                    }

                    _ => {}
                }
            },
            ScoreState::Running(start_time) => {
                let mut control_flow = ControlFlow::Continue(());
                while control_flow.is_continue() {
                    // Wait for message or for update period, whichever happens sooner
                    let msg_opt = with_timeout(update_period, SCORE_CHANNEL.receive())
                        .await
                        .ok();
                    control_flow = msg_opt.map_or_else(
                        // If timeout, keep looping (score value will be updated each loop)
                        || ControlFlow::Continue(()),
                        |msg| match msg {
                            // If `ScoreMsg::Stop` received, advance state and set loop exit condition
                            ScoreMsg::Stop => {
                                *self = ScoreState::Stopped;
                                ControlFlow::Break(())
                            }
                            // Otherwise, keep looping (score value will be updated each loop)
                            _ => ControlFlow::Continue(()),
                        },
                    );

                    // Update score value
                    let elapsed_millis = update_score(start_time);
                    // If score (i.e. game run time) => ~10s, notify  `game_loop`; it may wish to
                    // halt this run of the game
                    (elapsed_millis > MAX_GAME_SCORE_MS)
                        .then(|| async { GAME_CHANNEL.send(GameMsg::ScoreOverflow).await });
                }
            }
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
