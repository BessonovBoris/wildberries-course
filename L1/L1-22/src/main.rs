fn main() {
    let mut vec = Vec::from_iter(0..=10);
    let i = 10;

    if i >= vec.len() {
        println!("WARNING: Index out of bounds");
        return;
    }

    vec.remove(i);

    println!("{:#?}", vec);
}
