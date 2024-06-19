#![feature(never_type)]
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

use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Duration;

use crate::numeric_led_driver::DecimalSeparator;
pub use {
    button_driver::button_driver, game_loop::game_loop, now_playing_led_driver::now_playing_led_driver,
    score_driver::score_driver,
};

// Define ergonomic aliases for message-type-specific channel implementations
pub type GameChannel<const CAPACITY: usize = 1> =
    Channel<CriticalSectionRawMutex, message::GameMsg, CAPACITY>;
pub type GameReceiver<const CAPACITY: usize = 1> =
    Receiver<'static, CriticalSectionRawMutex, message::GameMsg, CAPACITY>;
pub type GameSender<const CAPACITY: usize = 1> =
    Sender<'static, CriticalSectionRawMutex, message::GameMsg, CAPACITY>;
pub type NowPlayingLedChannel<const CAPACITY: usize = 1> =
    Channel<CriticalSectionRawMutex, message::NowPlayingLedMsg, CAPACITY>;
pub type NowPlayingLedReceiver<const CAPACITY: usize = 1> =
    Receiver<'static, CriticalSectionRawMutex, message::NowPlayingLedMsg, CAPACITY>;
pub type NowPlayingLedSender<const CAPACITY: usize = 1> =
    Sender<'static, CriticalSectionRawMutex, message::NowPlayingLedMsg, CAPACITY>;
pub type NumericLedChannel<const CAPACITY: usize = 1> =
    Channel<CriticalSectionRawMutex, message::NumericLedMsg, CAPACITY>;
pub type NumericLedReceiver<const CAPACITY: usize = 1> =
    Receiver<'static, CriticalSectionRawMutex, message::NumericLedMsg, CAPACITY>;
pub type NumericLedSender<const CAPACITY: usize = 1> =
    Sender<'static, CriticalSectionRawMutex, message::NumericLedMsg, CAPACITY>;
pub type ScoreChannel<const CAPACITY: usize = 1> =
    Channel<CriticalSectionRawMutex, message::ScoreMsg, CAPACITY>;
pub type ScoreReceiver<const CAPACITY: usize = 1> =
    Receiver<'static, CriticalSectionRawMutex, message::ScoreMsg, CAPACITY>;
pub type ScoreSender<const CAPACITY: usize = 1> =
    Sender<'static, CriticalSectionRawMutex, message::ScoreMsg, CAPACITY>;

// How long to light each digit panel of the 4-panel LED display
pub const LED_DISPLAY_PERSISTENCE_DELAY: Duration = Duration::from_millis(3);

// Define maximum game score (9,999 ms is plenty of time to react and is all that will fit on screen
// without adding the complexity fo moving the decimal point and truncating ms).
pub const MAX_GAME_SCORE_MS: u16 = 9_999;

// Define maximum waitable `Duration` (`Timer::after()`) in `embassy-rs`.  Empirically derived.
pub const MAX_WAITABLE_DURATION: Duration = Duration::from_secs(1 << 45 - 1);

// How frequently to update the `score` value when `ScoreState<Running>`.
pub const SCORE_DRIVER_UPDATE_PERIOD: Duration = Duration::from_millis(1);

pub static DECIMAL_SEPARATOR: DecimalSeparator = DecimalSeparator::Thousands;
pub static GAME_CHANNEL: GameChannel = Channel::new();
pub static NOW_PLAYING_LED_CHANNEL: NowPlayingLedChannel = Channel::new();
pub static SCORE_CHANNEL: ScoreChannel = Channel::new();
pub static NUMERIC_LED_CHANNEL: NumericLedChannel = Channel::new();
pub static SCORE: AtomicU16 = AtomicU16::new(0);
