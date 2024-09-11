use std::thread;
use std::time::Duration;

// function to emulate long work(access to database, etc.)
fn expensive_work() {
    thread::sleep(Duration::from_secs(1));
}

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

fn main() {
    let vector_len = read_vector_capacity().unwrap();
    let nums = Vec::from_iter(1..=vector_len);
    let mut handles = Vec::new();  // JoinHandle vector to end tasks

    for i in nums {
        // start computing square in new thread
        handles.push(thread::spawn(move || {
            println!("{}", i * i)
        }));
    }

    expensive_work();

    // wait until all thread finish job
    for handle in handles {
        handle.join().unwrap();
    }
}
