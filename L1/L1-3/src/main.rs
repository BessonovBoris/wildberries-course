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
    let n = read_vector_capacity().unwrap();
    let arr= Vec::from_iter(1..=n);

    // create channel to send data from thread to main
    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = Vec::new();

    // create channels to calculate squares
    for i in arr {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            tx.send(i).unwrap();
        }))
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

    println!("Sum = {}", accumulate);
}