mod state;

use core::marker::PhantomData;
use core::sync::atomic::Ordering;
use embassy_executor::{SpawnError, Spawner};
use embassy_time::{Duration, Timer};

use crate::message::{GameMsg, NowPlayingLedMsg};
use crate::{
    GameReceiver, NowPlayingLedSender, NumericLedSender, ScoreSender, NOW_PLAYING_LED_CHANNEL,
    SCORE,
};
use state::{Idle, IdleTransitioning, Pausing, Running, RunningTransitioning, State};

pub struct Game<State> {
    spawner: Spawner,
    receiver: GameReceiver,
    now_playing_led_sender: NowPlayingLedSender,
    numeric_led_sender: NumericLedSender,
    score_sender: ScoreSender,
    phantom_data: PhantomData<State>,
}

impl Game<Idle> {
    pub fn new(
        spawner: Spawner,
        receiver: GameReceiver,
        now_playing_led_sender: NowPlayingLedSender,
        numeric_led_sender: NumericLedSender,
        score_sender: ScoreSender,
    ) -> Self {
        Self {
            spawner,
            receiver,
            now_playing_led_sender,
            numeric_led_sender,
            score_sender,
            phantom_data: PhantomData,
        }
    }

    #[embassy_executor::task]
    async fn advance(game: &Self) {
        game.process_button_down().await;
        game.process_button_up().await;
        game.random_pause().await;
    }

    async fn process_button_down(&self) {
        // Process `GameMsg::ButtonDown`
        loop {
            let msg = self.receiver.receive().await;

            match msg {
                GameMsg::ButtonDown => {
                    SCORE.store(0, Ordering::Relaxed);
                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_button_up(&self) {
        // Process `GameMsg::ButtonUp`
        loop {
            let msg = self.receiver.receive().await;

            match msg {
                GameMsg::ButtonUp => {
                    NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::On).await;
                    break;
                }
                _ => {}
            }
        }
    }

    async fn random_pause(&self) {
        // Pause for a random duration from (0.5..=5)s
        Timer::after(Duration::from_secs(3)).await; // TODO: Make random
    }
}

impl From<Game<Idle>> for Game<Running> {
    fn from(game_idle: Game<Idle>) -> Self {
        game_idle.spawner.spawn(Game::<Idle>::advance(&game_idle)).expect(
            "Internal error: Multiple instances of this task were spawned even though only one is \
                    possible.",
        );
        Self {
            spawner: game_idle.spawner,
            receiver: game_idle.receiver,
            now_playing_led_sender: game_idle.now_playing_led_sender,
            numeric_led_sender: game_idle.numeric_led_sender,
            score_sender: game_idle.score_sender,
            phantom_data: PhantomData,
        }
    }
}
