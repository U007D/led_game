#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameMsg {
    ButtonDown,
    ButtonUp,
    ScoreOverflow,
}
