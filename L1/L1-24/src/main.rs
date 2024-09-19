use std::collections::HashSet;

fn main() {
    let s = "asdfA".to_string();
    println!("{}", uniq_symbol(s));
}

fn uniq_symbol(s: String) -> bool {
    let mut set = HashSet::new();
    for c in s.to_lowercase().chars() {
        if !set.insert(c) {
            return false;
        }
    };

    true
}
