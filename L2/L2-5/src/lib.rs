use clap::Parser;
use regex::Regex;
use std::cmp::{max, min};

// config struct with flags
#[derive(Parser, Debug)]
pub struct Config {
    pub file_name: String,
    pattern: String,

    #[clap(short = 'A', default_value = "0")]
    after: usize,

    #[clap(short = 'B', default_value = "0")]
    before: usize,

    #[clap(short = 'C', default_value = "0")]
    context: usize,

    #[clap(short = 'c', default_value = "false")]
    count: bool,

    #[clap(short = 'i', default_value = "false")]
    ignore_case: bool,

    #[clap(short = 'v', default_value = "false")]
    invert: bool,

    #[clap(short = 'F', default_value = "false")]
    fixed: bool,

    #[clap(short = 'n', default_value = "false")]
    line_number: bool,
}

impl Config {
    pub fn new() -> Config {
        Config::parse()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

pub fn grep(lines: &[&str], config: &Config) -> Vec<String> {
    let mut output = vec![];
    let lowercase_pattern = config.pattern.to_lowercase();

    let pattern = Regex::new(if config.ignore_case {
        &lowercase_pattern
    } else {
        &config.pattern
    })
    .unwrap();

    let mut furthest_line = 0;  // furthest line in vector avoid repetitions
    let format = format(config.line_number);    // formater line

    for line in 0..lines.len() {
        // last line to add
        let after = max(config.context, config.after) + line;
        let after = min(after, lines.len() - 1);

        // first line to add
        let before = max(config.before, config.context);
        let mut before = line.checked_sub(before).unwrap_or_default();

        // move first line to avoid repetitions
        if before < furthest_line {
            before = furthest_line;
        }

        if config.fixed {
            if lines[line].contains(&config.pattern) ^ config.invert {  // A*!B + !A*B = A xor B ( A - contains, B - invert )
                add_lines(&mut output, &lines[before..=after], before+1, format);
            }
            furthest_line = after+1;    // move the furthest line
        } else if pattern.is_match(lines[line]) ^ config.invert {
            add_lines(&mut output, &lines[before..=after], before+1, format);
            furthest_line = after+1;
        }
    }

    if config.count {
        return vec![output.len().to_string()];
    }

    output
}

// add line number if necessary
fn format(line_number: bool) -> fn(line: &str, num: usize) -> String {
    if !line_number {
        return |line: &str, _num: usize| -> String {
            return line.to_string();
        }
    }

    |line: &str, num: usize| -> String {
        return format!("{}: {}", num, line);
    }
}

// add formated lines to output vector
fn add_lines(lines: &mut Vec<String>, after: &[&str], mut lines_num: usize, format: fn(line: &str, num: usize) -> String) {

    for line in after {
        let line = format(line, lines_num);
        lines_num += 1;
        lines.push(line);
    }
}
