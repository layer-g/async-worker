use std::marker::PhantomData;
use tokio::{task::JoinHandle, sync::mpsc::{Receiver, Sender}};

struct ReceiverStruct<M> {
    socket: zmq::Socket,
    _phantom: PhantomData<M>,
}


/// Trait for ability to create an instance of Self.
impl<M> ReceiverStruct<M>
where
    M: Into<zmq::Message>
{
    /// Create a new instance of self.
    fn new(ctx: &zmq::Context, endpoint: &str) -> Self {

        // create + bind socket
        let socket = ctx.socket(zmq::PUSH).expect("Failed to create PUSH socket");
        socket.bind(endpoint).expect("Failed to bind PUSH socket");
        Self { socket, _phantom: Default::default() }
    }

    /// Run loop to receive from socket.
    async fn run(&mut self, mut receiver: Receiver<M>) {
        while let Some(msg) = receiver.recv().await {
            self.socket.send::<zmq::Message>(msg.into(), 0).expect("Failed to send message")
        }
    }
}

pub trait Adapter {
    type M: Send + Sync + Into<zmq::Message> + 'static;
    /// Initialize this `Adapter`.
    // fn init(ctx: zmq::Context, endpoint: String) -> (Sender<<<Self as Adapter>::Actor as Actor>::Message>, JoinHandle<()>) {
    fn init(ctx: &zmq::Context, endpoint: &String) -> (Sender<Self::M>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = ReceiverStruct::<<Self as Adapter>::M>::new(ctx, endpoint);
        let handle = tokio::spawn(async move { actor.run(receiver).await });
        (sender, handle)
    }
}
