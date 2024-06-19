use crate::{
    GAME_CHANNEL, MAX_DELAY_MS, MIN_DELAY_MS, NOW_PLAYING_LED_CHANNEL, SCORE, SCORE_CHANNEL,
};
use core::sync::atomic::Ordering;
use embassy_time::{with_timeout, Duration, Instant};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use crate::message::{GameMsg, NowPlayingLedMsg, ScoreMsg};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    Stopped,
    Timing,
}

impl GameState {
    pub async fn advance(&mut self) {
        match *self {
            Self::Stopped => {
                // Await `ButtonDown` message
                self.process_stopped_button_down().await;

                // Await `ButtonUp` message
                self.process_stopped_button_up().await;

                *self = Self::Timing;
            }

            Self::Timing => {
                // Await `ButtonDown` message
                self.process_timing_button_down().await;

                // Await `ButtonUp` message
                self.process_timing_button_up().await;

                *self = Self::Stopped;
            }
        }
    }

    async fn process_stopped_button_down(&self) {
        // Process `GameMsg::ButtonDown`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            match msg {
                GameMsg::ButtonDown => {
                    SCORE.store(0, Ordering::Relaxed);
                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_stopped_button_up(&self) {
        // Process `GameMsg::ButtonUp`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            match msg {
                GameMsg::ButtonUp => {
                    NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::On).await;
                    self.random_pause().await;
                    SCORE_CHANNEL.send(ScoreMsg::Start).await;

                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_timing_button_down(&self) {
        // Process `GameMsg::ButtonDown`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            match msg {
                GameMsg::ButtonDown => {
                    SCORE_CHANNEL.send(ScoreMsg::Stop).await;
                    NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::Off).await;
                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_timing_button_up(&self) {
        // Process `GameMsg::ButtonUp`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            use GameMsg as Gm;
            match msg {
                Gm::ButtonUp => {
                    break;
                }
                _ => {}
            }
        }
    }

    async fn random_pause(&self) {
        let delay = SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EED).next_u64()
            % (MAX_DELAY_MS.saturating_sub(MIN_DELAY_MS)).saturating_add(MIN_DELAY_MS);
        // Pause for a random duration from (0.5..=5)s.  Eat all messages (button key presses)
        // during delay period
        let delay_period = Duration::from_millis(delay);
        let delay_start = Instant::now();
        loop {
            let elapsed_time = Instant::now().saturating_duration_since(delay_start);
            match elapsed_time >= delay_period {
                false => {
                    let _ =
                        with_timeout(Duration::from_millis(delay), GAME_CHANNEL.receive()).await;
                }
                true => break,
            }
        }
    }
}

// (_, GMsg::ScoreOverflow) => {
// SCORE_CHANNEL.send(SMsg::Stop).await;
// NOW_PLAYING_LED_CHANNEL.send(NpMsg::Off).await;
// state = GameState::Stopped;
// }
// }
// }
