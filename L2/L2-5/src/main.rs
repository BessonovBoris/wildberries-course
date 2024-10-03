use L2_5::grep;
use L2_5::Config;

fn main() {
    let config = Config::new();
    let content = std::fs::read_to_string(config.file_name.clone()).unwrap();
    let lines: Vec<&str> = content.split("\r\n").collect();

    let v = grep(&lines, &config);
    for i in v {
        println!("{}", i);
    }
}
