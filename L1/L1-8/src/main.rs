use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use dashmap::DashMap;

fn realisation_hashmap() {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for i in 0..10 {
        let map = map.clone();
        handles.push(std::thread::spawn(move || {
            // lock to safety insert and prevent data race
            map.lock().unwrap().insert(format!("Number-{i}"), i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", map.lock().unwrap());
}

fn realisation_dashmap() {
    let dm = DashMap::new();
    let dm = Arc::new(dm);

    let mut handles = Vec::new();
    for i in 0..5 {
        let dm_clone = dm.clone();
        handles.push(std::thread::spawn(move || {
            dm_clone.insert(format!("Number-{}", i), i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", dm);
}

fn main() {
    println!("Hash Map: ");
    realisation_hashmap();

    println!("Dash Map: ");
    realisation_dashmap();
}
