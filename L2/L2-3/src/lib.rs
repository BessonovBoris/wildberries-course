use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn sort(mut file: &File, reverse: bool, numeric_sort: bool, key: usize, unique: bool) {
    let mut input = "".to_string();
    file.read_to_string(&mut input).unwrap();

    let lines = input
        .split("\n")
        .map(|s| {
            if s[s.len() - 1..].eq("\r") {
                return s[..s.len() - 1].to_string();
            }

            s.parse().unwrap()
        })
        .collect::<Vec<String>>();

    let mut lines = lines
        .iter()
        .map(|s| s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    println!("{:?}", lines);
    println!();

    if unique {
        let mut set: HashSet<Vec<String>> = HashSet::new();
        let ref_set = &mut set;

        lines.iter().for_each(|x| { ref_set.insert(x.clone()); });

        lines = set.iter().cloned().collect::<Vec<Vec<String>>>();
    }

    let compare = |a: &Vec<String>, b: &Vec<String>| -> Ordering {
        let a = a.get(key).unwrap();
        let b = b.get(key).unwrap();

        if reverse {
            return a.cmp(&b);
        }

        a.cmp(&b)
    };

    lines.sort_by(compare);

    println!("{:?}", lines);
}
