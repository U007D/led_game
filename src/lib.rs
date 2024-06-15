#![no_std]

pub mod button_driver;
pub mod error;
mod game_loop;
mod message;
pub mod numeric_led_driver;
mod score_driver;
mod solo_led_driver;

use core::sync::atomic::AtomicU16;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

use crate::numeric_led_driver::DecimalSeparator;
pub use {
    button_driver::button_driver, game_loop::game_loop, message::Message,
    score_driver::score_driver, solo_led_driver::solo_led_driver,
};

pub static DECIMAL_SEPARATOR: DecimalSeparator = DecimalSeparator::Thousands;
pub static GAME_CHANNEL: Channel<CriticalSectionRawMutex, Message, 1> = Channel::new();
pub static SCORE_CHANNEL: Channel<CriticalSectionRawMutex, Message, 1> = Channel::new();
pub static SOLO_LED_CHANNEL: Channel<CriticalSectionRawMutex, Message, 1> = Channel::new();
pub static SCORE: AtomicU16 = AtomicU16::new(4321);
