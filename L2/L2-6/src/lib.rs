use clap::builder::TypedValueParser;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    pub file_name: String,

    #[clap(short='f', default_value="0")]
    fields: String,

    #[clap(short='d', default_value=" ")]
    delimiter: char,

    #[clap(short='s', default_value="false")]
    separated: bool,
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

pub fn cut(lines: &[&str], config: &Config) -> Result<Vec<Vec<String>>, String> {
    let mut output = vec![];
    let fields = config.fields.split(",").collect::<Vec<&str>>();   // parse filed by ',', later parse diapasons

    for line in lines {
        let mut output_line = vec![];   // formated line
        let split_line: Vec<&str> = line.split(config.delimiter).collect();

        // if len == 1 => no delimiter in line
        if config.separated && split_line.len() <= 1 {
            continue;
        }

        for field in &fields {
            if let Ok(num) = field.parse::<usize>() {
                if num < split_line.len() {
                    output_line.push(split_line[num].to_string());
                }
            } else {
                let range = field.split('-').collect::<Vec<&str>>();

                // try parse start of diapason
                let start = if let Ok(start) = range[0].parse::<usize>() {
                    start
                } else {
                    return Err(format!("Could not parse field: {}", field));
                };

                // try parse end of diapason
                let end = if let Ok(end) = range[1].parse::<usize>() {
                    end
                } else {
                    return Err(format!("Could not parse field: {}", field));
                };

                for i in start..=end {
                    if i >= split_line.len() {
                        break;
                    }

                    output_line.push(split_line[i].to_string());
                }
            }
        }

        output.push(output_line);
    }

    Ok(output)
}