mod game_state;

use game_state::GameState;

pub async fn game_loop() -> ! {
    let mut state = GameState::Stopped;

    loop {
        // Transition state from `Stopped` -> `Running`
        state.advance().await;
        // Transition state from `Running` -> `Stopped`
        state.advance().await;
    }
}
