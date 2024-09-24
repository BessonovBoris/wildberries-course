use L2_2::unpacking;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let s = unpacking(&input);
    println!("{}", s.unwrap());
}