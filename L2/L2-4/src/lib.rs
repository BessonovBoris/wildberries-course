use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn search_anagrams<'a, 'b>(dictionary: &'a [&'b str]) -> Box<HashMap<&'b str, Vec<String>>> {
    let mut answer = Box::new(HashMap::new());

    for word in dictionary.iter() {
        if answer.contains_key(word) {
            continue;
        }

        let lowercase_word = word.to_lowercase();
        let lowercase_word = lowercase_word.as_str();

        let mut anagrams = get_all_anagrams(lowercase_word);
        anagrams.sort();

        if anagrams.len() > 1 {
            answer.insert(*word, anagrams);
        }
    }

    answer
}

fn get_all_anagrams(word: &str) -> Vec<String> {
    let chars = word.chars().collect::<Vec<char>>();

    // generate all anagrams, but save only unique
    let unique_anagrams = chars
        .into_iter()
        .permutations(word.len())
        .map(|p| p.iter().collect::<String>())
        .collect::<HashSet<String>>();

    unique_anagrams.into_iter().collect()
}
