use async_workers::{RecvActor, EngineMessage, EngineError, SendActor, AdapterSend, AdapterRecv};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let ctx = zmq::Context::new();
    let endpoint = "tcp://127.0.0.1:6969".to_string();
    // actors
    let (messenger, send_handle) = SendActor::<EngineMessage>::init(&ctx, &endpoint);
    let (mut engine, recv_handle) = RecvActor::<EngineMessage, EngineError>::init(&ctx, &endpoint);

    // keep track of the number of messages
    let mut count = 0;
    let (stop_sender, mut stop_receiver) = tokio::sync::mpsc::channel(1);
    
    // spawn messenger task
    let messenger = tokio::spawn(async move {
        let format = format!("message: {count:?}");
        loop {
            tokio::select! {
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                    let msg = EngineMessage(Bytes::from(format.clone()));
                    let _ = messenger.send(msg).await;
                }

                _ = stop_receiver.recv() => break
            }
        }
        println!("messenger task complete");
    });

    loop {
        let interval = tokio::time::interval(std::time::Duration::from_millis(100));
        tokio::pin!(interval);

        tokio::select! {
            _ = interval.tick() => {
                println!("interval");
                if count == 10 {
                    println!("interval break. count: {count:?}");
                    break
                }
            }
    
            // received a message
            Some(msg) = engine.recv() => {
                println!("engine: {msg:?}");
                count += 1;
                if count == 10 {
                    println!("engine break. count: {count:?}");
                    break
                }
            }
    
            else => println!("all channels closed")
        }
    }

    let res = stop_sender.send(()).await;
    println!("res1: {res:?}");
    let res = messenger.await;
    println!("res2: {res:?}");
    send_handle.abort();
    let res = send_handle.await;
    println!("res3: {res:?}");
    recv_handle.abort();
    let res = recv_handle.await;
    println!("res4: {res:?}");
    println!("count: {count:#?}");
}
