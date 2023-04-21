use std::marker::PhantomData;
use tokio::{sync::mpsc::{Sender, Receiver}, task::JoinHandle};
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
    pub async fn run(&mut self, sender: Sender<M>) {
        loop {
            println!("receiver loop");
            match RecvActor::<M, E>::receive_message(&self.socket) {
                Ok(msg) => {
                    println!("receive loop - msg okay");
                    let msg = msg.try_into().unwrap();
                    // sender.send(msg).await.expect("Failed to send msg on tokio channel");
                    // todo!()
                    let send_handle = tokio::spawn({
                        let sender = sender.clone();
                        println!("receive loop - inside sub spawn");
                        async move { let _ = sender.send(msg).await; println!("receive loop - inside async move") }
                    });
                },
                Err(e) => {
                    println!("receive loop - msg Err");
                    panic!("{e:?}");
                }
            }
        }
        // loop {
        //     tokio::select! {
        //         msg = RecvActor::<M, E>::receive_message(&self.socket) => {
        //             match msg {
        //                 Ok(msg) => {
        //                     let msg = msg.try_into().unwrap();
        //                     tokio::spawn({
        //                         let sender = sender.clone();
        //                         async move { let _ = sender.send(msg).await; }
        //                     });
        //                 },
        //                 Err(e) => panic!("{e:?}"),
        //             }
        //         },
        //         _ = stop_receiver => {
        //             break;
        //         }
        //     }
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
    fn init(ctx: &zmq::Context, endpoint: &str) -> (Receiver<Self::M>, JoinHandle<()>) {
        let (
            sender,
            receiver
        ) = tokio::sync::mpsc::channel::<Self::M>(100);
        let mut actor = RecvActor::<Self::M, Self::Error>::new(ctx, endpoint);
        let handle = tokio::spawn(async move{actor.run(sender).await });

        (receiver, handle)
    }
}
