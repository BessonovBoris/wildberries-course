fn main() {
    let s = reverse_words("snow dog sun".to_string());
    println!("{}", s);
}

fn reverse_words(s: String) -> String {
    s.split(' ').rev().collect::<Vec<&str>>().join(" ")
}
