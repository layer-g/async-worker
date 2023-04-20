use async_trait::async_trait;
use tokio::{task::JoinHandle, sync::mpsc::{Receiver, Sender}};

struct ReceiverStruct {
    socket: zmq::Socket,
}

/// Trait for ability to create an instance of Self.
#[async_trait]
pub trait Actor
where
    Self: Sized + Send + Sync + 'static
{
    /// Message type for the Receiver
    type Message: Send + Sync + Into<zmq::Message>;

    /// Create a new instance of self.
    fn new(ctx: zmq::Context, endpoint: String) -> Self;

    /// Provide the context used to bind zmq sockets.
    fn ctx(&self) -> &zmq::Context;

    /// Endpoint for zmq sockets.
    fn endpoint(&self) -> &str;

    /// Run loop to receive from socket.
    async fn run(&mut self, mut receiver: Receiver<Self::Message>) {
        // create + bind socket
        let socket = self.ctx().socket(zmq::PUSH).expect("Failed to create PUSH socket");
        socket.bind(self.endpoint()).expect("Failed to bind PUSH socket");

        while let Some(msg) = receiver.recv().await {
            socket.send::<zmq::Message>(msg.into(), 0).expect("Failed to send message")
        }
        todo!()
    }
}

pub trait Adapter
where
    Self: Sized
{
    type Actor: Actor;

    // /// Initialize new instance of self
    // fn new(sender: Sender<<<Self as Adapter>::Actor as Actor>::Message>) -> Self {
    //     Self { sender }
    // }

    /// Initialize this `Adapter`.
    fn init(ctx: zmq::Context, endpoint: String) -> Sender<<<Self as Adapter>::Actor as Actor>::Message> {
    // fn init(ctx: zmq::Context, endpoint: String) -> (Self, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<<<Self as Adapter>::Actor as Actor>::Message>(100);
        let mut actor: Self::Actor = Actor::new(ctx, endpoint);
        tokio::spawn(async move { actor.run(receiver).await });
        // Self { sender }
        // todo!()
        sender
    }
}
