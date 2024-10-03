use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;
use clap::Parser;
use serde_json::{json, Value};

#[derive(Parser, Debug)]
pub struct Config {
    pub file_name: String,

    #[clap(short='t', default_value="1")]
    threads: usize,
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

pub fn lf(mut file: File, config: &Config) -> Value {
    let start_time = Instant::now();

    let mut output = HashMap::new();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("can't read file");

    let lines:Vec<String> = content.split("\r\n").map(|x| x.to_string()).collect();
    let lines= Arc::new(lines);

    let delta = div_ceil(lines.len(), config.threads);
    let mut start = 0;
    let mut handles = Vec::new();

    while start < lines.len() {
        let end = min(start + delta, lines.len());
        let lines = Arc::clone(&lines);

        handles.push(std::thread::spawn(move || {
            count_chars(&lines[start..end])
        }));
        start += delta;
    }

    for handle in handles {
        let map = handle.join().unwrap();
        merge_hash_maps(&mut output, &map);
    }

    let duration = start_time.elapsed().as_millis() as f32 / 1000.0;

    json!({
        "elapsed": format!("{:.3} s", duration),
        "result": output
    })
}

pub fn count_chars(lines: &[String]) -> HashMap<char, usize> {
    let mut output = HashMap::new();

    for line in lines {
        let chars = count_chars_in_line(line);
        merge_hash_maps(&mut output, &chars);
    }

    output
}

fn count_chars_in_line(line: &str) -> HashMap<char, usize> {
    let mut output: HashMap<char, usize> = HashMap::new();

    for char in line.chars() {
        if !char.is_alphabetic() {
            continue;
        }

        if let Some(count) = output.get_mut(&char) {
            *count += 1;
        } else {
            output.insert(char, 1);
        }
    }

    output
}

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn merge_hash_maps(map1: &mut HashMap<char, usize>, map2: &HashMap<char, usize>) {
    for (char, count) in map2.into_iter() {
        if let Some(map1_count) = map1.get_mut(&char) {
            *map1_count += count;
        } else {
            map1.insert(*char, *count);
        }
    }
}