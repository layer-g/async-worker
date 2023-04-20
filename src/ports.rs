use super::{ReceiverStruct, /*SenderStruct*/};
use tokio::{task::JoinHandle, sync::mpsc::{Receiver, Sender}};

pub trait AdapterRecv
where zmq::Message: From<<Self as AdapterRecv>::InternalMessage>
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
        let mut actor = ReceiverStruct::new(ctx, endpoint);
        let handle = tokio::spawn(async move { actor.run(receiver).await });
        (sender, handle)
    }
}

// pub trait AdapterSend {
//     type M: Send + Sync + Into<zmq::Message> + 'static;
//     /// Initialize this `Adapter`.
//     // fn init(ctx: zmq::Context, endpoint: String) -> (Sender<<<Self as Adapter>::Actor as Actor>::Message>, JoinHandle<()>) {
//     fn init(ctx: &zmq::Context, endpoint: &String) -> (Sender<Self::M>, JoinHandle<()>) {
//         let (
//             sender,
//             receiver
//         ) = tokio::sync::mpsc::channel::<Self::M>(100);
//         let mut actor = SenderStruct::<<Self as AdapterSend>::M>::new(ctx, endpoint);
//         let handle = tokio::spawn(async move { actor.run(receiver).await });
//         (sender, handle)
//     }
// }
