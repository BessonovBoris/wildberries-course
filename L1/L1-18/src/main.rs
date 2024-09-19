use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let s = s.graphemes(true).rev().collect::<String>();

    println!("{}", s);
}
