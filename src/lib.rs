#![feature(error_in_core)]
#![no_std]

use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIN_0;
use crate::message::Message;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_sync::mutex::Mutex;

// pub mod button_driver;
pub mod error;
pub mod led_driver;
mod message;

pub static CHANNEL: Channel<CriticalSectionRawMutex, Message, 1> = Channel::new();
pub static LED: Mutex<CriticalSectionRawMutex, Option<Output<'static, PIN_0>>> = Mutex::new(None);