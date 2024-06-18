use crate::game_old::state_new::StateNew::Pausing;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StateNew {
    Idle,
    IdleTransitioning,
    Pausing,
    Running,
    RunningTransitioning,
}
