use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn read_vector_capacity() -> Result<usize, String> {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string())
    }

    let num = match input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(err) => return Err(err.to_string())
    };

    Ok(num)
}

fn realisation_with_channel(n: usize, arr: Vec<i32>) -> i32 {
    // create channel to send data from thread to main
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    // create channels to calculate squares
    for i in arr {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            tx.send(i).unwrap();
        }));
    }

    let mut accumulate = 0;
    for _ in 0..n {
        let num = rx.recv().unwrap();
        accumulate += num;
    }

    // wait until all jobs done
    for handle in handles {
        handle.join().unwrap();
    }

    accumulate
}

fn realisation_with_arc_mutex(n: usize, arr: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();

    // add arc to send data from different channels
    let tx = Arc::new(Mutex::new(tx));
    let mut handles = Vec::new();

    for i in arr {
        let tx = Arc::clone(&tx);
        handles.push(thread::spawn(move || {
            let tx = tx.lock().unwrap();
            tx.send(i).unwrap();
        }));
    }

    let mut accumulate = 0;
    for _ in 0..n {
        let num = rx.recv().unwrap();
        accumulate += num;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    accumulate
}

fn main() {
    let n = read_vector_capacity().unwrap();
    let arr= Vec::from_iter(1..=(n as i32));

    let sum = realisation_with_channel(n, arr);
    println!("Sum = {}", sum);

    // let sum = realisation_with_arc_mutex(n, arr);
    // println!("Sum = {}", sum);
}