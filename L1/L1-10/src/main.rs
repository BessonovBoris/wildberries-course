use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let (calculus_tx, calculus_rx) = mpsc::channel();

    let vector_reader = thread::spawn(move || {
        let n = 10;
        let vec = Vec::from_iter(0..=n);

        for i in vec {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let calculating = thread::spawn(move || {
        loop {
            let num = match rx.recv() {
                Ok(n) => n,
                Err(_) => break,
            };

            calculus_tx.send(num * num).unwrap();
        }
    });

    let output = thread::spawn(move || {
        loop {
            let num = match calculus_rx.recv() {
                Ok(n) => n,
                Err(_) => break,
            };

            println!("{}", num);
        }
    });

    vector_reader.join().unwrap();
    calculating.join().unwrap();
    output.join().unwrap();
}
