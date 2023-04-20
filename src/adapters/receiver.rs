use std::marker::PhantomData;
use tokio::sync::mpsc::Receiver;

use crate::ports::AdapterRecv;

pub struct ReceiverStruct<M> {
    socket: zmq::Socket,
    _phantom: PhantomData<M>,
}

/// Receive external ZMQ messages
impl<M> ReceiverStruct<M>
where
    M: zmq::Sendable,// Into<zmq::Message>// + From<<Self as AdapterRecv>::InternalMessage>
    zmq::Message: From<M>
{
    /// Create a new instance of self.
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {

        // create + bind socket
        let socket = ctx.socket(zmq::PUSH).expect("Failed to create PUSH socket");
        socket.bind(endpoint).expect("Failed to bind PUSH socket");
        Self { socket, _phantom: Default::default() }
    }

    /// Run loop to receive from socket.
    pub async fn run(&mut self, mut receiver: Receiver<M>) {
        while let Some(msg) = receiver.recv().await {
            self.socket.send::<zmq::Message>(msg.into(), 0).expect("Failed to send message")
        }
    }
}


// pub struct ReceiverStruct<IM, EM> {
//     socket: zmq::Socket,
//     _phantom: PhantomData<(IM, EM)>,
// }

// /// Receive external ZMQ messages
// impl<IM, EM> ReceiverStruct<IM, EM>
// where
//     IM: Into<EM>,// Into<zmq::Message>// + From<<Self as AdapterRecv>::InternalMessage>
//     zmq::Message: From<IM>
// {
//     /// Create a new instance of self.
//     pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {

//         // create + bind socket
//         let socket = ctx.socket(zmq::PUSH).expect("Failed to create PUSH socket");
//         socket.bind(endpoint).expect("Failed to bind PUSH socket");
//         Self { socket, _phantom: Default::default() }
//     }

//     /// Run loop to receive from socket.
//     pub async fn run(&mut self, mut receiver: Receiver<IM>) {
//         while let Some(msg) = receiver.recv().await {
//             self.socket.send::<zmq::Message>(msg.into(), 0).expect("Failed to send message")
//         }
//     }
// }
