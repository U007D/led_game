use crate::error::message;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Stop,
    Start,
}

impl From<Message> for u8 {
    fn from(value: Message) -> Self {
        use Message as M;
        match value {
            M::Stop => 0,
            M::Start => 1,
        }
    }
}

impl TryFrom<u8> for Message {
    type Error = message::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Message as M;
        match value {
            0 => Ok(M::Stop),
            1 => Ok(M::Start),
            _ => Err(Self::Error::InvalidMessageDeserializationValue(value)),
        }
    }
}
