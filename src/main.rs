use async_workers::{RecvActor, ports::{AdapterRecv, AdapterSend}, EngineMessage, EngineError, SendActor};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let ctx = zmq::Context::new();
    let endpoint = "tcp://127.0.0.1:6969".to_string();
    
    let (messenger, _handle) = SendActor::<EngineMessage>::init(&ctx, &endpoint);
    let (mut engine, _handle) = RecvActor::<EngineMessage, EngineError>::init(&ctx, &endpoint);

    let (time, mut timer) = tokio::sync::mpsc::channel(1);

    let mut count = 0;
    
    // spawn messenger task
    tokio::spawn(async move {
        let format = format!("message: {count:?}");
        loop {
            let msg = EngineMessage(Bytes::from(format.clone()));
            let _ = messenger.send(msg).await;
        }
    });

    // spawn timer task
    tokio::spawn(async move {
        loop {
            let _ = tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            let _ = time.send(()).await;
        }
    });

    loop {
        tokio::select! {
            // timer
            Some(ok) = timer.recv() => {
                println!("timer went off: {ok:?}");
                if count == 10 {
                    drop(timer);
                    break
                }
            }
    
            // received a message
            Some(msg) = engine.recv() => {
                println!("received engine message: {msg:?}");
                count += 1;
                if count == 10 {
                    drop(engine);
                    break
                }
            }
    
            else => println!("all channels closed")
        }
    }

    println!("count: {count:#?}");
}
