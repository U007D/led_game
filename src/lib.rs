#![feature(error_in_core)]
#![no_std]

pub mod button_driver;
pub mod error;
mod led_driver;
mod message;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

pub use {button_driver::button_driver, led_driver::led_driver, message::Message};

pub static CHANNEL: Channel<CriticalSectionRawMutex, Message, 1> = Channel::new();
