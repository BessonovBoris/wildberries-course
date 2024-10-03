use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};

///
/// Attempts to sort given file and returns sorted one
///
/// # Errors
///
/// This function will return an error if `new_file_path` is invalid,
/// flag `numeric_sort` is `true`, but line is not a number,
/// `key` is more than lines length
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use L2_3::sort;
///
/// fn main() {
///     let file = std::fs::File::open(Path::new(
///         "./src/test_file"
///     ))
///     .unwrap();
///
///     let sorted_file = sort(&file, "./src/test_file_sorted".to_string(), false, false, 1, false).unwrap_or_else(|err| {
///         println!("{}", err);
///         std::process::exit(1);
///     });
/// }
/// ```
///
pub fn sort(mut file: &File, new_file_path: String, reverse: bool, numeric_sort: bool, key: usize, unique: bool) -> Result<File, String> {
    let mut input = "".to_string();
    file.read_to_string(&mut input).unwrap();

    let mut lines = format_lines(&*input);
    lines = check_lines(lines, key, numeric_sort)?;

    // remove repeated lines
    if unique {
        let mut set: HashSet<Vec<&str>> = HashSet::new();
        let ref_set = &mut set;

        lines.iter().for_each(|x| { ref_set.insert(x.clone()); });
        lines = set.iter().cloned().collect::<Vec<Vec<&str>>>();
    }

    // make closure to use `key` and `reverse` values here. Compare two lines by key
    let compare = |a: &Vec<&str>, b: &Vec<&str>| -> Ordering {
        if numeric_sort {
            let a = a.get(key).unwrap().parse::<i32>().unwrap();
            let b = b.get(key).unwrap().parse::<i32>().unwrap();

            if reverse {
                return a.cmp(&b).reverse();
            }

            return a.cmp(&b)
        }

        let a = a.get(key).unwrap();
        let b = b.get(key).unwrap();

        if reverse {
            return a.cmp(&b).reverse();
        }

        a.cmp(&b)
    };

    lines.sort_by(compare);

    let output_file = File::create(new_file_path).unwrap();
    write_to_file(&output_file, lines);

    Ok(output_file)
}

fn format_lines(input: &str) -> Vec<Vec<&str>> {
    // remove lines separators (for CRLF)
    let lines = input
        .split("\r\n")
        .collect::<Vec<&str>>();

    // split by space
    let lines = lines
        .iter()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    lines
}

fn check_lines(lines: Vec<Vec<&str>>, key: usize, numeric_sort: bool) -> Result<Vec<Vec<&str>>, String> {
    // check lines length and if there are all numbers
    for line in lines.iter() {
        if line.len() <= key {
            return Err("key greater than line length".to_string());
        }

        if numeric_sort {
            if let Err(_) = line[key].parse::<i32>() {
                return Err("not a number".to_string());
            }
        }
    }

    Ok(lines)
}

fn write_to_file(mut file: &File, lines: Vec<Vec<&str>>) {
    // make bytes to write to file
    let bytes_lines = lines.iter().map(|s| {
        Vec::from(s.join(" ").as_bytes())
    }).collect::<Vec<Vec<u8>>>();

    // write bytes and `\n` at the end of line
    for line in bytes_lines.iter() {
        file.write_all(line).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    };
}