use std::sync::mpsc;
use std::time::Duration;

fn read_working_time() -> Result<u64, String> {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string())
    }

    let num = match input.trim().parse::<u64>() {
        Ok(num) => num,
        Err(err) => return Err(err.to_string())
    };

    Ok(num)
}

#[tokio::main]
async fn main() {
    let working_time = read_working_time().unwrap();
    let (tx, rx) = mpsc::channel();
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(working_time)).await;
        shutdown_tx.send(()).unwrap();
    });
    tokio::spawn(producer(tx, shutdown_rx));
    tokio::spawn(consumer(rx)).await.unwrap();
}

async fn producer(tx: mpsc::Sender<i32>, shutdown_rx: mpsc::Receiver<()>) {
    let mut i = 1;
    loop {
        if let Ok(_) = shutdown_rx.try_recv() {
            break;
        }

        tx.send(i).unwrap();
        i += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn consumer(rx: mpsc::Receiver<i32>) {
    loop {
        let num = match rx.recv() {
            Ok(n) => n,
            Err(_) => break,
        };
        println!("Received {}", num);
    }
}
