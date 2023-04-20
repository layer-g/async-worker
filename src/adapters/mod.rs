mod receiver;
mod sender;
pub use sender::SenderStruct;
pub use receiver::ReceiverStruct;
use crate::ports::{AdapterSend, /*AdapterSend*/};
use bytes::Bytes;
use thiserror::Error;

#[derive(Debug)]
pub struct EngineMessage(Bytes);

#[derive(Debug, Clone, Copy, Error)]
pub enum EngineError {
    #[error("Default engine error.")]
    Unknown,

    #[error("ZMQ error receiving {0:?}")]
    Zmq(#[from] zmq::Error),
}

// impl From<zmq::Message> for EngineMessage {
//     fn from(value: zmq::Message) -> Self {
//         Self(value.to_vec().into())
//     }
// }

impl From<EngineMessage> for zmq::Message {
    fn from(value: EngineMessage) -> Self {
        Self::from(value.0.to_vec())
    }
}

impl TryFrom<zmq::Message> for EngineMessage {
    type Error = EngineError;

    fn try_from(value: zmq::Message) -> Result<Self, Self::Error> {
        Ok(Self(Bytes::default()))
    }
}

pub struct EngineAdapter;

impl AdapterSend for EngineAdapter {
    // type ExternalMessage = zmq::Message;
    type InternalMessage = EngineMessage;
}
