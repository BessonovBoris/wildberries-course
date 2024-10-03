use std::fs;
use std::io::Write;
use reqwest;

fn main() {
    let url = input();
    let response = reqwest::blocking::get(url).expect("Can't get page"); // get page data
    let html_content = response
        .text()
        .unwrap()
        .split("><")
        .collect::<Vec<_>>()
        .join(">\n<");  // parse page to write it

    fs::write("./page.html", html_content)
        .expect("Can't write file");
}

fn input() -> String {
    let mut input = String::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    let input = input.trim();

    input.to_string()
}