#![feature(async_closure, never_type)]
#![no_std]

pub mod button_driver;
pub mod error;

// pub mod game;
// pub mod game_old;
mod game_loop;
mod message;
mod now_playing_led_driver;
pub mod numeric_led_driver;
mod score_driver;
pub mod shared_consts;

use core::sync::atomic::AtomicU16;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Duration;

use crate::numeric_led_driver::DecimalSeparator;
pub use {
    button_driver::button_driver, game_loop::game_loop,
    now_playing_led_driver::now_playing_led_driver, score_driver::score_driver,
};

// How long to light each digit panel of the 4-panel LED display
pub const LED_DISPLAY_PERSISTENCE_DELAY: Duration = Duration::from_millis(3);

// Maximum time the game will wait after starting the game to begin timing the player's response
pub const MAX_DELAY_MS: u64 = 5_000;

// Minimum time the game will wait after starting the game to begin timing the player's response
pub const MIN_DELAY_MS: u64 = 500;

// Halt compilation if the `MIN_DELAY` is not less than `MAX_DELAY`
const _VALID_DELAY_ASSERTION: () = assert!(MIN_DELAY_MS < MAX_DELAY_MS);

// Define maximum game score (9,999 ms is plenty of time to react and is all that will fit on screen
// without adding the complexity fo moving the decimal point and truncating ms).
pub const MAX_GAME_SCORE_MS: u16 = 9_999;

// Define maximum waitable `Duration` (`Timer::after()`) in `embassy-rs`.  Empirically derived.
pub const MAX_WAITABLE_DURATION: Duration = Duration::from_secs(1 << 45 - 1);

// How frequently to update the `score` value when `ScoreState<Running>`.
pub const SCORE_DRIVER_UPDATE_PERIOD: Duration = Duration::from_millis(1);

// Maximum number of message possible to enqueue in channels for inter-task communication
const CAPACITY: usize = 1;

pub static DECIMAL_SEPARATOR: DecimalSeparator = DecimalSeparator::Thousands;
pub static SCORE: AtomicU16 = AtomicU16::new(0);

pub static GAME_CHANNEL: Channel<CriticalSectionRawMutex, message::GameMsg, CAPACITY> =
    Channel::new();
pub static NOW_PLAYING_LED_CHANNEL: Channel<
    CriticalSectionRawMutex,
    message::NowPlayingLedMsg,
    CAPACITY,
> = Channel::new();
pub static SCORE_CHANNEL: Channel<CriticalSectionRawMutex, message::ScoreMsg, CAPACITY> =
    Channel::new();
pub static NUMERIC_LED_CHANNEL: Channel<CriticalSectionRawMutex, message::NumericLedMsg, CAPACITY> =
    Channel::new();
