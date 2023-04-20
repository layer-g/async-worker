
use std::marker::PhantomData;
use tokio::sync::mpsc::Receiver;

use crate::ports::AdapterSend;

use super::EngineMessage;

pub struct SenderStruct<M> {
    socket: zmq::Socket,
    _phantom: PhantomData<M>,
}

impl<M> AdapterSend for SenderStruct<M> {
    type InternalMessage = EngineMessage;
}

/// Receive external ZMQ messages
impl<M> SenderStruct<M>
where
    M: zmq::Sendable,// Into<zmq::Message>// + From<<Self as AdapterRecv>::InternalMessage>
    // zmq::Message: From<M>
{
    /// Create a new instance of self.
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {
        // create + bind socket
        let socket = ctx.socket(zmq::PUSH).expect("Failed to create PUSH socket");
        socket.bind(endpoint).expect("Failed to bind PUSH socket");
        Self { socket, _phantom: Default::default() }
    }

    /// Run loop to receive from internal channel and send over socket.
    pub async fn run(&mut self, mut receiver: Receiver<M>) {
        while let Some(msg) = receiver.recv().await {
            self.socket.send::<M>(msg.into(), 0).expect("Failed to send message")
        }
    }
}