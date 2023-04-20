use std::marker::PhantomData;
use tokio::sync::mpsc::Sender;

use crate::ports::AdapterRecv;

use super::{EngineMessage, EngineError};

pub struct ReceiverStruct<M> {
    socket: zmq::Socket,
    _phantom: PhantomData<M>,
}

impl<M> AdapterRecv for ReceiverStruct<M> {
    type Error = EngineError;
    type M = EngineMessage;
}


/// Send zmq messages
impl<M> ReceiverStruct<M>
where
    // M: TryFrom<zmq::Message, Error = <Self as AdapterRecv>::Error> + std::fmt::Debug,
    M: Send + Sync + zmq::Sendable + std::fmt::Debug + TryFrom<zmq::Message, Error = <Self as AdapterRecv>::Error>,
{
    /// Create a new instance of self.
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {
        // create + bind socket
        let socket = ctx.socket(zmq::PULL).expect("Failed to create PUSH socket");
        socket.connect(endpoint).expect("Failed to bind PUSH socket");
        Self { socket, _phantom: Default::default() }
    }

    fn receive_message(&self) -> Result<M, <Self as AdapterRecv>::Error> {
        self.socket.recv_msg(0)?.try_into()
    }

    /// Run loop to receive from socket.
    pub async fn run(&mut self, sender: Sender<M>) {
        // while let Some(msg) = receiver.recv().await {
        //     self.socket.send::<zmq::Message>(msg.into(), 0).expect("Failed to send message")
        // }
        loop {
            match self.receive_message() {
                Ok(msg) => sender.send(msg).await.expect("Failed to send msg on tokio channel"),
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}
