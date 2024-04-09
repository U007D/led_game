#![feature(error_in_core, never_type)]
#![no_std]

mod error;
mod game;

// Suppress spurious `unused_imports` warning (i.e. these imports *are* used!):
//      `defmt_rtt` defines several symbols required by the linker.
//      `panic_probe` defines the custom panic handler required by this `no_std` crate.
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
pub use {error::Result, game::Game};
