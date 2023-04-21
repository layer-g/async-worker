use super::{SenderStruct, ReceiverStruct};
use pin_utils::pin_mut;
use tokio::{task::JoinHandle, sync::mpsc::{Receiver, Sender}};

pub trait AdapterSend
// where zmq::Message: From<<Self as AdapterRecv>::InternalMessage>
{
    // type ExternalMessage: From<Self::InternalMessage>;
    type InternalMessage: Send + Sync + 'static + zmq::Sendable; //Into<Self::ExternalMessage> + 'static;
    /// Initialize this `Adapter`.
    // fn init(ctx: zmq::Context, endpoint: String) -> (Sender<<<Self as Adapter>::Actor as Actor>::Message>, JoinHandle<()>) {
    fn init(ctx: &zmq::Context, endpoint: &str) -> (Sender<Self::InternalMessage>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::InternalMessage>(100);
        let mut actor = SenderStruct::new(ctx, endpoint);
        let handle = tokio::spawn(async move { actor.run(receiver).await });
        (sender, handle)
    }
}

pub trait AdapterRecv {
    type Error: Send + Sync + From<zmq::Error> + std::fmt::Debug + 'static;
    type M: Send + Sync + zmq::Sendable + std::fmt::Debug + TryFrom<zmq::Message, Error = Self::Error> + 'static;
    /// Initialize this `Adapter`.
    // fn init(ctx: zmq::Context, endpoint: String) -> (Sender<<<Self as Adapter>::Actor as Actor>::Message>, JoinHandle<()>) {
    fn init(ctx: &zmq::Context, endpoint: &str) -> (Receiver<Self::M>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = ReceiverStruct::<Self::M, Self::Error>::new(ctx, endpoint);
        // let socket = ctx.socket(zmq::PULL).expect("Failed to create PULL socket");
        // socket.connect(endpoint).expect("Failed to connect PULL socket");
        // pin_mut!(actor);

        // let handle = tokio::spawn(actor.run(sender) );
        let socket = ctx.socket(zmq::PULL).expect("Failed to create PULL socket");
        socket.connect(endpoint).expect("Failed to connect PULL socket");
        let socket = Box::new(socket);
        // pin_mut!(socket);
        // tokio::pin!(socket);
        let socket = Box::pin(socket);
        let mut actor = ReceiverStruct::<Self::M, Self::Error>::new(ctx, endpoint);
        let handle = actor.run(sender);


        // (receiver, handle)
        todo!()
    }
}
