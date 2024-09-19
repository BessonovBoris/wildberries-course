use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = reverse_words("hello world".to_string());
    println!("{}", s);
}

fn reverse_words(s: String) -> String {
    let vec_words = s.split(' ').collect::<Vec<&str>>();
    let mut ans = String::new();

    for word in vec_words.iter() {
        let s = word.graphemes(true).rev().collect::<String>();
        ans.push_str(format!("{} ", s).as_str());
    }
    ans.pop();

    ans
}
