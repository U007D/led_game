use super::GameState;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Idle;
impl GameState for Idle {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IdleTransitioning;
impl GameState for IdleTransitioning {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pausing;
impl GameState for Pausing {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Running;
impl GameState for Running {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RunningTransitioning;
impl GameState for RunningTransitioning {}
