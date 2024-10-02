use std::path::Path;
use L2_3::sort;

fn main() {
    let file = std::fs::File::open(Path::new(
        "C:\\Users\\boris\\RustroverProjects\\wildberries-course\\L2\\L2-3\\src\\test_file",
    ))
    .unwrap();
    sort(&file, false, false, 0, true);
}
