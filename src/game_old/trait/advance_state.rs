use crate::game_old::{Game, GameState};

pub trait AdvanceState<T, U>
    where
        T: GameState,
        U: GameState,
{
    async fn advance_state(self) -> Game<U>;
}

