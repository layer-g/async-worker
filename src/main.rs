use async_workers::EngineAdapter;
use async_workers::ports::Adapter;

fn main() {
    let ctx = zmq::Context::new();
    let endpoint = "something".to_string();
    let _channel = EngineAdapter::init(ctx, endpoint);
}
