use L2_7::{Config, lf};

fn main() {
    let config = Config::new();
    let res = lf(std::fs::File::open(&config.file_name).unwrap(), &config);

    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}
