use std::collections::HashMap;
use L2_4::search_anagrams;

#[test]
fn regular_anagram() {
    let input = ["ab", "cd"];
    let mut expected_output = HashMap::new();
    expected_output.insert("ab", vec!["ab".to_string(), "ba".to_string()]);
    expected_output.insert("cd", vec!["cd".to_string(), "dc".to_string()]);

    let actual_output = search_anagrams(&input);

    assert_eq!(*actual_output, expected_output);
}

#[test]
fn repeated_words() {
    let input = ["ab", "ab"];
    let mut expected_output = HashMap::new();
    expected_output.insert("ab", vec!["ab".to_string(), "ba".to_string()]);

    let actual_output = search_anagrams(&input);

    assert_eq!(*actual_output, expected_output);
}

#[test]
fn one_anagram_test() {
    let input = ["lll", "ab"];
    let mut expected_output = HashMap::new();
    expected_output.insert("ab", vec!["ab".to_string(), "ba".to_string()]);

    let actual_output = search_anagrams(&input);

    assert_eq!(*actual_output, expected_output);
}