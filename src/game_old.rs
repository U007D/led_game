mod state;
mod state_new;
mod r#trait;

use core::marker::PhantomData;
use core::sync::atomic::Ordering;

use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_time::Timer;

use crate::{
    button_driver,
    error::Result,
    message::ScoreMsg,
    message::{GameMsg, NowPlayingLedMsg},
    now_playing_led_driver,
    numeric_led_driver::{numeric_led_driver, NumericLedPins},
    score_driver, GameReceiver, NowPlayingLedSender, NumericLedSender, ScoreSender,
    LED_DISPLAY_PERSISTENCE_DELAY, NOW_PLAYING_LED_CHANNEL, SCORE, SCORE_CHANNEL,
    SCORE_DRIVER_UPDATE_PERIOD,
};
use r#trait::{AdvanceState, GameState};
use state::{Idle, IdleTransitioning, Pausing, Running, RunningTransitioning};
use state_new::StateNew;

pub struct GameNew {
    state: StateNew,
}

pub struct Game<State> {
    game_receiver: GameReceiver,
    now_playing_led_sender: NowPlayingLedSender,
    numeric_led_sender: NumericLedSender,
    score_sender: ScoreSender,
    phantom_data: PhantomData<State>,
}

impl Game<Idle> {
    pub fn new(
        spawner: Spawner,
        game_receiver: GameReceiver,
        now_playing_led_sender: NowPlayingLedSender,
        numeric_led_sender: NumericLedSender,
        score_sender: ScoreSender,
    ) -> Self {
        Self {
            game_receiver,
            now_playing_led_sender,
            numeric_led_sender,
            score_sender,
            phantom_data: PhantomData,
        }
    }
}

impl AdvanceState<Idle, IdleTransitioning> for Game<Idle> {
    async fn advance_state(self) -> Game<IdleTransitioning> {
        loop {
            let msg = self.game_receiver.receive().await;

            match msg {
                GameMsg::ButtonDown => {
                    SCORE.store(0, Ordering::Relaxed);
                    break Game::<IdleTransitioning>::from(self);
                }
                _ => {}
            }
        }
    }
}

impl AdvanceState<IdleTransitioning, Pausing> for Game<IdleTransitioning> {
    async fn advance_state(self) -> Game<Pausing> {
        loop {
            let msg = self.game_receiver.receive().await;

            match msg {
                GameMsg::ButtonUp => {
                    NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::On).await;
                    break self.into();
                }
                _ => {}
            }
        }
    }
}

impl AdvanceState<Pausing, Running> for Game<Pausing> {
    async fn advance_state(self) -> Game<Running> {
        NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::On).await;
        Timer::after_secs(3).await; // To do: update to random delay (0.5..=5)s
        SCORE_CHANNEL.send(ScoreMsg::Start).await;
        self.into()
    }
}

impl AdvanceState<Running, RunningTransitioning> for Game<Running> {
    async fn advance_state(self) -> Game<RunningTransitioning> {
        loop {
            let msg = self.game_receiver.receive().await;

            use GameMsg as Gm;
            use NowPlayingLedMsg as NplMsg;
            match msg {
                Gm::ButtonUp => {
                    NOW_PLAYING_LED_CHANNEL.send(NplMsg::On).await;
                    break self.into();
                }
                _ => {}
            }
        }
    }
}

impl AdvanceState<RunningTransitioning, Idle> for Game<RunningTransitioning> {
    async fn advance_state(self) -> Game<Idle> {
        loop {
            let msg = self.game_receiver.receive().await;

            use GameMsg as Gm;
            use NowPlayingLedMsg as NplMsg;
            match msg {
                Gm::ButtonUp => {
                    NOW_PLAYING_LED_CHANNEL.send(NplMsg::On).await;
                    break self.into();
                }
                _ => {}
            }
        }
    }
}

impl From<Game<Idle>> for Game<IdleTransitioning> {
    fn from(game: Game<Idle>) -> Self {
        Self {
            game_receiver: game.game_receiver,
            now_playing_led_sender: game.now_playing_led_sender,
            numeric_led_sender: game.numeric_led_sender,
            score_sender: game.score_sender,
            phantom_data: PhantomData,
        }
    }
}

impl From<Game<IdleTransitioning>> for Game<Pausing> {
    fn from(game: Game<IdleTransitioning>) -> Self {
        Self {
            game_receiver: game.game_receiver,
            now_playing_led_sender: game.now_playing_led_sender,
            numeric_led_sender: game.numeric_led_sender,
            score_sender: game.score_sender,
            phantom_data: PhantomData,
        }
    }
}

pub async fn run(game_idle: Game<Idle>, spawner: Spawner) -> Result<!> {
    spawn_tasks(spawner)?;
    loop {
        let game_idle_transitioning = game_idle.advance_state().await;
        let game_pausing = game_idle_transitioning.advance_state().await;
        let game_running = game_pausing.advance_state().await;
        let game_running_transitioning = game_running.advance_state().await;
        let game_idle = game_running_transitioning.advance_state().await;
    }
}

fn spawn_tasks(spawner: Spawner) -> Result<()> {
    let p = embassy_rp::init(Default::default());
    let numeric_led_pins = NumericLedPins::new(
        p.PIN_1, p.PIN_2, p.PIN_3, p.PIN_4, p.PIN_5, p.PIN_6, p.PIN_7, p.PIN_8, p.PIN_9, p.PIN_10,
        p.PIN_11, p.PIN_12,
    );

    spawner
        .spawn(numeric_led_driver(
            numeric_led_pins,
            LED_DISPLAY_PERSISTENCE_DELAY,
        ))
        .map_err(|err| (err, "Error spawning numeric_led_driver task"))?;

    spawner
        .spawn(score_driver(SCORE_DRIVER_UPDATE_PERIOD))
        .map_err(|err| (err, "Error spawning score_driver task"))?;

    spawner
        .spawn(now_playing_led_driver(p.PIN_0.degrade()))
        .map_err(|err| (err, "Error spawning now_playing_led_driver task"))?;

    spawner
        .spawn(button_driver(p.PIN_13.degrade()))
        .map_err(|err| (err, "Error spawning button_driver task"))?;

    Ok(())
}
