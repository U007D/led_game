#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameMode {
    #[default]
    Stopped,
    Timing,
}