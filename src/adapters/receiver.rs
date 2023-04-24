use std::marker::PhantomData;
use futures::StreamExt;
use tmq::Multipart;
use tokio::{sync::mpsc::{Sender, Receiver}, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use super::{EngineMessage, EngineError};

pub struct RecvActor<M, E> {
    socket: tmq::pull::Pull,
    _phantom: PhantomData<(M, E)>,
}

impl<M, E> AdapterRecv for RecvActor<M, E> {
    type Error = EngineError;
    type M = EngineMessage;
}

/// Send tmq messages
impl<M, E> RecvActor<M, E>
where
    // M: Send + Sync /* + tmq::Sendable*/ + std::fmt::Debug + TryFrom<tmq::Message, Error = E> + 'static,
    M: Send + Sync /* + tmq::Sendable*/ + std::fmt::Debug + TryFrom<Multipart, Error = E> + 'static,
    E: Send + Sync + From<tmq::TmqError> + std::fmt::Debug + 'static,
{
    /// Create a new instance of self.
    pub fn new(ctx: &tmq::Context, endpoint: &str) -> Self {
        // create + bind socket
        // let socket = ctx.socket(tmq::PULL).expect("Failed to create PUSH socket");
        let socket = tmq::pull(&ctx);
        let socket = socket.connect(endpoint).expect("Failed to bind PUSH socket");

        Self { socket, _phantom: Default::default() }
    }

    // pub async fn receive_message(socket: &tmq::pull::Pull) -> AsyncResult<M> {
    //     println!("waiting for socket to receive...");
    //     let m: M = socket.recv_msg(0)?.try_into().unwrap();
    //     // m.map_err(|e| std::io::Error{ repr: e})
    //     // socket.poll(events, timeout_ms) // maybe try to poll instead?
    //     Ok(m)
    // }

    // pub fn receive_message(socket: &tmq::Socket) -> Result<M, E> {
    //     println!("waiting for socket to receive...");
        // socket.recv_msg(0)?.try_into()
        // socket.poll(events, timeout_ms) // maybe try to poll instead?
    // }

    /// Run loop to receive from socket.
    pub async fn run(&mut self, sender: Sender<M>, token: CancellationToken) {
        tokio::pin!(token);
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    println!("token cancelled in tmq loop");
                    break
                }

                Some(msg) = self.socket.next() => {
                    println!("tmq_receiver recvd!");
                    let msg = msg.expect("Tmq pull socket receives a valid message.");
                    let msg = msg.try_into().expect("Tmq pull socket should convert to M if VecDeque length == 1.");
                    let res = sender.send(msg).await;
                    match res {
                        Ok(_) => (),
                        Err(_) => break,
                    }
                }
            }
        }
    }
}

pub trait AdapterRecv {
    type M: Send
        + Sync
        + std::fmt::Debug
        + TryFrom<tmq::Multipart, Error = Self::Error>
        + 'static;
    type Error: Send
        + Sync
        + From<tmq::TmqError>
        + std::fmt::Debug
        + 'static;

    /// Initialize this `Adapter`.
    fn init(ctx: &tmq::Context, endpoint: &str) -> (Receiver<Self::M>, JoinHandle<()>, CancellationToken) {
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
