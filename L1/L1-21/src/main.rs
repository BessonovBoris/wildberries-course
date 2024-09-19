use num_bigint::BigInt;

fn main() {
    let a = BigInt::from((2 << 20) + 10);
    let b = BigInt::from((2 << 20) + 20);

    let multiply = a.clone() * b.clone();
    println!("a * b = {}", multiply);

    let quotient = a.clone() / b.clone();
    println!("a / b = {}", quotient);

    let sum = a.clone() + b.clone();
    println!("a + b = {}", sum);

    let difference = a.clone() - b.clone();
    println!("a - b = {}", difference);
}