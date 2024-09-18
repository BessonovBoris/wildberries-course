use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let number_of_workers = 4;
    let (rx, tx) = channel();
    let tx = Arc::new(Mutex::new(tx)); // Arc and Mutex to use receiver in many threads
    let mut handles = Vec::with_capacity(number_of_workers);

    // Make n receivers
    for i in 1..=number_of_workers {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            receiver_worker(tx, i);
        }));
    }

    // Make sender
    thread::spawn(move || {
        sender_worker(rx);
    });

    // don't stop program until all workers finish
    for handle in handles {
        handle.join().unwrap();
    }
}

fn receiver_worker(tx: Arc<Mutex<Receiver<String>>>, id: usize) {
    loop {
        let message_result = tx.lock().unwrap().recv();

        match message_result {
            Ok(message) => println!("Thread {} received message: {}", id, message),
            Err(_) => {     // channel closed
                println!("Thread {} exiting", id);
                break;
            }
        }
    }
}

fn sender_worker(rx: Sender<String>) {
    let mut message_number = 1;
    loop {
        let message = format!("Message number: {}", message_number);
        rx.send(message).unwrap();

        message_number += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
