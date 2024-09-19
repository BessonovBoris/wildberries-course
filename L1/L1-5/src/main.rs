use tokio::sync::{watch};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    // channel to signal shutdown to all workers
    let (shutdown_tx, shutdown_rx) = watch::channel(());

    // make worker task
    let worker_shutdown_rx = shutdown_rx.clone();
    let worker_handle = tokio::spawn(async move {
        worker(worker_shutdown_rx).await;
    });

    // make task to listen ctrl+c signal
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        shutdown_tx.send(()).unwrap();
        println!("Ctrl+C received, shutting down");
    };

    // wait for ctrl+c signal or finish worker
    // when one of task finished, other task is deleted
    tokio::select! {
        _ = shutdown_signal => {},
        _ = worker_handle => {},
    }
}

async fn worker(mut shutdown_rx: watch::Receiver<()>) {
    loop {
        tokio::select! {
            // task to handle ctrl+c signal
            _ = shutdown_rx.changed() => {
                println!("Worker shutting down");
                break;
            }
            // main work
            _ = time::sleep(Duration::from_secs(1)) => {
                println!("Worker doing work");
            }
        }
    }
}