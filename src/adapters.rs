use crate::ports::Adapter;
use bytes::Bytes;

pub struct EngineMessage(Bytes);

impl From<zmq::Message> for EngineMessage {
    fn from(value: zmq::Message) -> Self {
        Self(value.to_vec().into())
    }
}

impl From<EngineMessage> for zmq::Message {
    fn from(value: EngineMessage) -> Self {
        Self::from(value.0.to_vec())
    }
}

pub struct EngineAdapter;

impl Adapter for EngineAdapter { type M = EngineMessage; }
