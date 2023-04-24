mod receiver;
mod sender;
use std::ops::Deref;

pub use sender::{SendActor, AdapterSend};
pub use receiver::{RecvActor, AdapterRecv};
use bytes::Bytes;
use thiserror::Error;


#[derive(Debug)]
pub struct EngineMessage(pub Bytes);

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Default engine error.")]
    Unknown,

    #[error("tmq error receiving {0:?}")]
    Tmq(#[from] tmq::TmqError),

    #[error("TMQ Multipart message longer than expected: {0}")]
    MessageLength(usize),
}

// impl From<tmq::Message> for EngineMessage {
//     fn from(value: tmq::Message) -> Self {
//         Self(value.to_vec().into())
//     }
// }

impl From<EngineMessage> for tmq::Message {
    fn from(value: EngineMessage) -> Self {
        Self::from(value.0.to_vec())
    }
}

impl From<EngineMessage> for tmq::Multipart {
    fn from(value: EngineMessage) -> Self {
        Self::from(vec![value])
    }
}

// impl From<tmq::Multipart> for EngineMessage {
//     fn from(value: tmq::Multipart) -> Self {

//     }
// }

impl TryFrom<tmq::Multipart> for EngineMessage {
    type Error = EngineError;

    fn try_from(mut value: tmq::Multipart) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            return Err(EngineError::MessageLength(value.len()))
        }
        // safe to unwrap: already confirmed length == 1
        Ok(Self(value.pop_front().unwrap().deref().to_owned().into()))
    }
}

pub struct EngineAdapter;

impl AdapterSend for EngineAdapter {
    // type ExternalMessage = tmq::Message;
    type M = EngineMessage;
}
