use L2_2::unpacking;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // remove end line symbol
    input.pop();
    input.pop();

    let s = unpacking(&input);
    println!("{}", s.unwrap());
}