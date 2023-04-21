use std::{marker::PhantomData, pin::Pin, future::Future};
use pin_utils::pin_mut;
use tokio::{sync::mpsc::Sender, task::JoinHandle, pin};

use crate::ports::AdapterRecv;

use super::{EngineMessage, EngineError};

// pub struct ReceiverStruct<'a, M, E> {
pub struct ReceiverStruct<M, E> {
    // socket: Pin<&'a mut zmq::Socket>,
    ctx: zmq::Context,
    endpoint: String,
    _phantom: PhantomData<(M, E)>,
}

// impl<'a, M, E> AdapterRecv for ReceiverStruct<'a, M, E> {
impl<M, E> AdapterRecv for ReceiverStruct<M, E> {
    type Error = EngineError;
    type M = EngineMessage;
}

/// Send zmq messages
// impl<'a, M, E> ReceiverStruct<'a, M, E>
impl<M, E> ReceiverStruct<M, E>
where
    // M: TryFrom<zmq::Message, Error = <Self as AdapterRecv>::Error> + std::fmt::Debug,
    M: Send + Sync + zmq::Sendable + std::fmt::Debug + TryFrom<zmq::Message, Error = E> + 'static,
    E: Send + Sync + From<zmq::Error> + std::fmt::Debug,
{
    /// Create a new instance of self.
    pub fn new(ctx: &zmq::Context, endpoint: &str) -> Self {
        // create + bind socket

        // let socket = ctx.socket(zmq::PULL).expect("Failed to create PUSH socket");
        // socket.connect(endpoint).expect("Failed to bind PUSH socket");
        // pin_mut!(socket);
        // Self { socket, _phantom: Default::default() }

        let ctx = ctx.clone();
        let endpoint = endpoint.to_owned();
        Self { ctx, endpoint, _phantom: Default::default() }
    }

    fn ctx(&self) -> &zmq::Context {
        &self.ctx
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    // fn receive_message(&self) -> Result<M, <Self as AdapterRecv>::Error> {
    // fn receive_message(socket: Pin<&zmq::Socket>) -> Result<M, E> {
    pub fn receive_message(socket: &zmq::Socket) -> Result<M, E> {
        socket.recv_msg(0)?.try_into()
    }

    /// Run loop to receive from socket.
    // pub async fn run(self: Pin<&mut Self>, sender: Sender<M>, socket: &zmq::Socket) {
    // pub async fn run(self: Pin<&mut Self>, sender: Sender<M>) {
    // pub async fn run(&mut self, sender: Sender<M>, socket: &Pin<&mut zmq::Socket>) -> dyn Future<Output = ()> + Send + 'static {
    // pub async fn run(&mut self, sender: Sender<M>) {
    pub fn run(&mut self, sender: Sender<M>) -> JoinHandle<()> {
    // pub async fn run(&mut self, sender: Sender<M>, socket: &zmq::Socket) {
        // let (tx, rx)  = tokio::sync::mpsc::channel(1);
        let socket = self.ctx().clone().socket(zmq::PULL).expect("Failed to create PULL socket");
        socket.connect(self.endpoint().clone()).expect("Failed to connect PULL socket");

        tokio::spawn(async move {
            loop {
                match ReceiverStruct::<M, E>::receive_message(&socket) {
                // match socket.recv_msg(0) {
                    Ok(msg) => {
                        let msg = msg.try_into().unwrap();
                        // sender.send(msg).await.expect("Failed to send msg on tokio channel");
                        // todo!()
                        tokio::spawn({
                            let sender = sender.clone();
                            async move { let _ = sender.send(msg).await; }
                        });
                    },
                    Err(e) => panic!("{e:?}"),
                }
            }
        })
        // };
    }
}
