use L2_6::Config;
use L2_6::cut;

fn main() {
    let config = Config::new();
    let content = std::fs::read_to_string(config.file_name.clone()).unwrap();
    let lines: Vec<&str> = content.split("\r\n").collect();

    println!("{:?}", config);
    println!("{:?}", lines);
    println!();

    match cut(&lines, &config) {
        Ok(line) => {
            for v in line {
                for line in v {
                    print!("{} ", line);
                }
                println!();
            }
        }

        Err(err) => println!("{}", err),
    }
}
