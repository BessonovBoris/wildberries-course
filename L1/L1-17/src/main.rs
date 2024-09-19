use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    // safety increment
    fn increment(&self) {
        let mut value = self.value.lock().unwrap();
        *value += 1;
    }

    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

fn main() {
    // Arc for clone to many threads
    let counter = Arc::new(Counter::new());
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..4 {
                counter.increment();
                thread::sleep(Duration::from_millis(500));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", counter.get());
}