#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Message {
    ButtonUp,
    ButtonDown,
    SoloLedOn,
    SoloLedOff,
    StartTimer,
    StopTimer,
}
