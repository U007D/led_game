#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;

use led_game::Game;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut game = Game::new();
    // PANIC SAFETY //
    // `fn run() -> Result<!, Error>` only returns if there was an error (`Ok` path is divergent),
    // so `unwrap_err()` cannot panic.
    let err = game
        .run()
        .await
        .expect_err("Internal Error: `Game::run()` should never return success, but did.");

    defmt::error!("Game exited with error {}.", err);

    // Fast-blink LED forever to signify error condition to user
    loop {
        game.toggle_led();
        Timer::after_millis(100).await;
    }
}
