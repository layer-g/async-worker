use std::{marker::PhantomData, collections::VecDeque};
use futures::SinkExt;
use tmq::Multipart;
use tokio::{sync::mpsc::{Receiver, Sender}, task::JoinHandle};
use super::EngineMessage;

pub struct SendActor<M> {
    socket: tmq::push::Push,
    _phantom: PhantomData<M>,
}

impl<M> AdapterSend for SendActor<M> {
    type M = EngineMessage;
}

/// Receive external tmq messages
impl<M> SendActor<M>
where
    M: Into<Multipart>,
{
    /// Create a new instance of self.
    pub fn new(ctx: &tmq::Context, endpoint: &str) -> Self {
        // create + bind socket
        // let socket = ctx.socket(tmq::pushPUSH).expect("Failed to create PUSH socket");
        let socket = tmq::push(&ctx);
        let socket = socket.bind(endpoint).expect("Failed to bind PUSH socket");
        Self { socket, _phantom: Default::default() }
    }

    /// Run loop to receive from internal channel and send over socket.
    pub async fn run(&mut self, mut receiver: Receiver<M>) {
        while let Some(msg) = receiver.recv().await {
            let res = self.socket.send(msg.into()).await;
            match res {
                Ok(_) => (),
                Err(e) => println!("Error pushing msg in socket: {e:?}"),
            }
        }
    }
}

pub trait AdapterSend
{
    type M: Send + Sync + Into<Multipart> + 'static;// + tmq::Sendable; //Into<Self::ExternalMessage> + 'static;

    /// Initialize this `Adapter`.
    fn init(ctx: &tmq::Context, endpoint: &str) -> (Sender<Self::M>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = SendActor::new(ctx, endpoint);
        let handle = tokio::spawn(async move { actor.run(receiver).await });
        (sender, handle)
    }
}
