#![feature(async_closure, never_type)]
#![no_std]
pub mod error;
pub mod numeric_led_driver;

use embassy_time::Duration;

// Define maximum waitable `Duration` (`Timer::after()`) in `embassy-rs`.  Empirically derived.
pub const MAX_WAITABLE_DURATION: Duration = Duration::from_secs(1 << 45 - 1);
