use super::{SenderStruct, ReceiverStruct};
use tokio::{task::JoinHandle, sync::mpsc::{Receiver, Sender}};

pub trait AdapterSend
// where zmq::Message: From<<Self as AdapterRecv>::InternalMessage>
{
    // type ExternalMessage: From<Self::InternalMessage>;
    type InternalMessage: Send + Sync + 'static + zmq::Sendable; //Into<Self::ExternalMessage> + 'static;
    /// Initialize this `Adapter`.
    // fn init(ctx: zmq::Context, endpoint: String) -> (Sender<<<Self as Adapter>::Actor as Actor>::Message>, JoinHandle<()>) {
    fn init(ctx: &zmq::Context, endpoint: &String) -> (Sender<Self::InternalMessage>, JoinHandle<()>) {
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
    type Error;
    // type M: Send + Sync + zmq::Sendable + std::fmt::Debug + TryFrom<zmq::Message, Error = Self::Error> + 'static;
    type M: Send + Sync + zmq::Sendable + std::fmt::Debug + TryFrom<zmq::Message, Error = Self::Error> + 'static;
    /// Initialize this `Adapter`.
    // fn init(ctx: zmq::Context, endpoint: String) -> (Sender<<<Self as Adapter>::Actor as Actor>::Message>, JoinHandle<()>) {
    fn init(ctx: &zmq::Context, endpoint: &String) -> (Sender<Self::M>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = ReceiverStruct::new(ctx, endpoint);
        let handle = tokio::spawn(async move { actor.run(sender).await });
        (sender, handle)
    }
}
