use crate::ports::{Actor, Adapter};
use async_trait::async_trait;
use bytes::Bytes;

pub struct EngineMessage(Bytes);

impl From<zmq::Message> for EngineMessage {
    fn from(value: zmq::Message) -> Self {
        Self(value.to_vec().into())
    }
}

impl From<EngineMessage> for zmq::Message {
    fn from(value: EngineMessage) -> Self {
        Self::from(value.0.to_vec())
    }
}

pub struct EngineActor {
    ctx: zmq::Context,
    endpoint: String,
}

#[async_trait]
impl Actor for EngineActor {

    type Message = EngineMessage;

    fn new(ctx: zmq::Context, endpoint: String) -> Self {
        Self { ctx, endpoint }
    }

    fn ctx(&self) -> &zmq::Context {
        &self.ctx
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

pub struct EngineAdapter;

impl Adapter for EngineAdapter {
    type Actor = EngineActor;
}

fn test() {
    let ctx = zmq::Context::new();
    let endpoint = "something".to_string();
    let channel = EngineAdapter::init(ctx, endpoint);
}