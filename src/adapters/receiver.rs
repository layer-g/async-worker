use std::marker::PhantomData;
use tokio::{sync::mpsc::{Sender, Receiver}, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use super::{EngineMessage, EngineError};

pub struct RecvActor<M, E> {
    socket: zmq::Socket,
    _phantom: PhantomData<(M, E)>,
}

impl<M, E> AdapterRecv for RecvActor<M, E> {
    type Error = EngineError;
    type M = EngineMessage;
}

/// Send zmq messages
impl<M, E> RecvActor<M, E>
where
    M: Send + Sync + zmq::Sendable + std::fmt::Debug + TryFrom<zmq::Message, Error = E> + 'static,
    E: Send + Sync + From<zmq::Error> + std::fmt::Debug,
{
    /// Create a new instance of self.
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {
        // create + bind socket
        let socket = ctx.socket(zmq::PULL).expect("Failed to create PUSH socket");
        socket.connect(endpoint).expect("Failed to bind PUSH socket");

        Self { socket, _phantom: Default::default() }
    }

    pub fn receive_message(socket: &zmq::Socket) -> Result<M, E> {
        socket.recv_msg(0)?.try_into()
    }

    /// Run loop to receive from socket.
    pub async fn run(&mut self, sender: Sender<M>, token: CancellationToken) {
        tokio::pin!(token);
        loop {
            println!("receiver loop");
            match RecvActor::<M, E>::receive_message(&self.socket) {
                Ok(msg) => {
                    println!("receive loop - msg okay");
                    let msg = msg.try_into().unwrap();
                    // sender.send(msg).await.expect("Failed to send msg on tokio channel");
                    // todo!()
                    tokio::spawn({
                        let sender = sender.clone();
                        let token = token.clone();
                        println!("receive loop - inside sub spawn");
                        async move {
                            tokio::select! {
                                _ = sender.send(msg) => println!("sent"),//.await.expect("receive loop - inside async move");
                                _ = token.cancelled() => println!("cancelled"),
                            }
                        }
                    });
                }
                Err(e) => {
                    panic!("Error receiving zmq socket: {e:?}");
                }
            }
        }
    }
}

pub trait AdapterRecv {
    type M: Send
        + Sync
        + zmq::Sendable
        + std::fmt::Debug
        + TryFrom<zmq::Message, Error = Self::Error>
        + 'static;
    type Error: Send
        + Sync
        + From<zmq::Error>
        + std::fmt::Debug
        + 'static;

    /// Initialize this `Adapter`.
    fn init(ctx: &zmq::Context, endpoint: &str) -> (Receiver<Self::M>, JoinHandle<()>, CancellationToken) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = RecvActor::<Self::M, Self::Error>::new(ctx, endpoint);
        let cancel_token = CancellationToken::new();
        let handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move{actor.run(sender, cancel_token).await }
        });

        (receiver, handle, cancel_token)
    }
}
