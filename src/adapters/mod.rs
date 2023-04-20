mod receiver;
// mod sender;
pub use receiver::ReceiverStruct;
// pub use sender::SenderStruct;
use crate::ports::{AdapterRecv, /*AdapterSend*/};
use bytes::Bytes;

pub struct EngineMessage(Bytes);

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

pub struct EngineAdapter;

impl AdapterRecv for EngineAdapter {
    // type ExternalMessage = zmq::Message;
    type InternalMessage = EngineMessage;
}
