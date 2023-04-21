use std::marker::PhantomData;
use tokio::{sync::mpsc::{Receiver, Sender}, task::JoinHandle};
use super::EngineMessage;

pub struct SendActor<M> {
    socket: zmq::Socket,
    _phantom: PhantomData<M>,
}

impl<M> AdapterSend for SendActor<M> {
    type M = EngineMessage;
}

/// Receive external ZMQ messages
impl<M> SendActor<M>
where
    M: zmq::Sendable,
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

pub trait AdapterSend
{
    type M: Send + Sync + 'static + zmq::Sendable; //Into<Self::ExternalMessage> + 'static;

    /// Initialize this `Adapter`.
    fn init(ctx: &zmq::Context, endpoint: &str) -> (Sender<Self::M>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = SendActor::new(ctx, endpoint);
        let handle = tokio::spawn(async move { actor.run(receiver).await });
        (sender, handle)
    }
}
