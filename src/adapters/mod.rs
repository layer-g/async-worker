mod receiver;
mod sender;
use std::ops::Deref;

pub use sender::SendActor;
pub use receiver::RecvActor;
use crate::ports::{AdapterSend, /*AdapterSend*/};
use bytes::Bytes;
use thiserror::Error;

#[derive(Debug)]
pub struct EngineMessage(pub Bytes);

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
        Ok(Self(value.deref().to_owned().into()))
    }
}

pub struct EngineAdapter;

impl AdapterSend for EngineAdapter {
    // type ExternalMessage = zmq::Message;
    type M = EngineMessage;
}
