#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScoreMode {
    #[default]
    Stopped,
    Running,
}