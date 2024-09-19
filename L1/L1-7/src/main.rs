use std::sync::mpsc;
use tokio::select;

async fn realisation_with_channel() {
    let (tx, rx) = mpsc::channel();

    // start example worker
    let worker = std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("{}", msg);
        }

        println!("Worker finished");
    });

    tx.send(1).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    tx.send(2).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // close channel -> end worker
    drop(tx);

    worker.join().unwrap();
}

async fn realisation_with_cancellation_token() {
    let token = tokio_util::sync::CancellationToken::new();
    let worker_token = token.clone();

    // start example worker
    let worker = tokio::spawn(async move {
        loop {
            select! {
                _ = worker_token.cancelled() => {
                    println!("Worker finished");
                    break
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {
                    println!("Doing work");
                }
            }
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // send signal to token -> end worker
    token.cancel();

    worker.await.unwrap();
}

#[tokio::main]
async fn main() {
    realisation_with_channel().await;
    realisation_with_cancellation_token().await;
}
