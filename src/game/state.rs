pub trait State {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Idle;
impl State for Idle {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IdleTransitioning;
impl State for IdleTransitioning {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pausing;
impl State for Pausing {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Running;
impl State for Running {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RunningTransitioning;
impl State for RunningTransitioning {}
