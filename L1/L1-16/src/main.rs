fn main() {
    let data = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let target = 11;

    let result = data.binary_search(&target);

    match result {
        Ok(index) => println!("Found {} at index {}", target, index),
        Err(_) => println!("{} not found in the array", target),
    }
}