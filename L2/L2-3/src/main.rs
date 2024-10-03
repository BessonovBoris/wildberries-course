use std::path::Path;
use L2_3::sort;

fn main() {
    let file = std::fs::File::open(Path::new(
        "./src/test_file"
    ))
    .unwrap();

    let _ = sort(&file, "./src/test_file_sorted".to_string(), false, false, 0, false).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });
}
